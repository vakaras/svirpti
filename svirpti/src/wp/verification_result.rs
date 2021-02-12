use crate::context::Context;
use crate::{errors::SvirptiResult, SmtContext};
use std::collections::{BTreeMap, HashMap, HashSet};
use svirpti_smt::solvers::{SatResult, SmtSolver};
use svirpti_vir::common::cfg::Cfg;
use svirpti_vir::common::expression::{UnaryOperationHelpers, VariableHelpers};
use svirpti_vir::{low, smt};

#[derive(Debug)]
pub struct Model {
    /// We use a `BTreeMap` here because we want to have deterministic
    /// iteration.
    pub variables: BTreeMap<low::VariableSymbol, smt::Value>,
}

impl Model {
    fn new<C: Context>(smt_context: &SmtContext<C>, model: &smt::Model) -> Self {
        // TODO: Get the variable version map and use it select which versions
        // of the variables were active at the failure point.
        let all_variables: HashSet<_> = smt_context
            .vir
            .variables
            .iter()
            .map(|variable| variable.name.clone())
            .collect();
        let mut variables = BTreeMap::new();
        for item in &model.items {
            let name = smt_context.context.resolve_low_variable(&item.name);
            if all_variables.contains(&name) {
                assert!(
                    variables.insert(name, item.value.clone()).is_none(),
                    "duplicate key"
                );
            }
        }
        Self { variables }
    }
}

#[derive(Debug)]
pub struct VerificationError {
    /// The assertion that failed.
    pub failing_assertion: low::LabelSymbol,
    /// The basic blocks that led to the failing assertion. Note that this is
    /// one of the traces that goes via the reported `labels`. There could be
    /// more than one if, for example, a passed if statement did not contribute
    /// to the failure.
    pub trace: Vec<low::BasicBlockId>,
    /// The labels reported by the SMT solver.
    pub labels: Vec<low::LabelSymbol>,
    /// The model returned by the SMT solver.
    pub model: Model,
}

impl VerificationError {
    fn new<C: Context>(
        smt_context: &SmtContext<C>,
        failing_assertion: smt::LabelSymbol,
        trace: Vec<smt::LabelSymbol>,
        model: &smt::Model,
    ) -> Self {
        let failing_assertion = smt_context.context.resolve_low_label(&failing_assertion);
        let trace: Vec<_> = trace
            .into_iter()
            .map(|label| smt_context.context.resolve_low_label(&label))
            .collect();
        // A map to track the path we took.
        let mut came_from = HashMap::new();
        // A label we expect to see next if we start to walk in that basic
        // block.
        let mut expected_next_label = HashMap::new();
        if trace.is_empty() {
            expected_next_label.insert(smt_context.vir.entry_block(), trace.len());
        } else {
            expected_next_label.insert(smt_context.vir.entry_block(), 0);
        }
        let mut final_block = None;
        for (id, basic_block) in smt_context.vir.walk() {
            let mut expected_label_id = expected_next_label[&id];
            let mut expected_label = trace.get(expected_label_id).unwrap_or(&failing_assertion);
            for statement in &basic_block.statements {
                if Some(expected_label) == statement.get_label() {
                    expected_label_id += 1;
                    expected_label = trace.get(expected_label_id).unwrap_or(&failing_assertion);
                    if expected_label == &failing_assertion {
                        final_block = Some(id);
                        break;
                    }
                }
            }
            for successor in &basic_block.successors {
                let current_label_id = expected_next_label
                    .entry(*successor)
                    .or_insert(expected_label_id);
                if *current_label_id <= expected_label_id {
                    *current_label_id = expected_label_id;
                    came_from.insert(successor, id);
                }
            }
        }
        let mut current_block = final_block.unwrap();
        let mut basic_block_trace = vec![current_block];
        while let Some(new_current_block) = came_from.get(&current_block) {
            current_block = *new_current_block;
            basic_block_trace.push(current_block);
        }
        basic_block_trace.reverse();

        Self {
            failing_assertion,
            model: Model::new(smt_context, model),
            trace: basic_block_trace,
            labels: trace,
        }
    }
}

/// Uses the approach described in [Generating error traces from
/// verification-condition
/// counterexamples](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/12/krml120.pdf)
/// to generate all errors.
pub(crate) fn get_all_errors<S: SmtSolver, C: Context>(
    mut failure: svirpti_smt::VerificationFailure<S>,
    smt_context: &SmtContext<C>,
) -> SvirptiResult<Vec<VerificationError>> {
    let investigator = failure.get_investigator();
    let labels = investigator.get_labels(smt_context)?;
    let mut model = investigator.get_model(smt_context)?;
    eprintln!("Labels: {:?} model: {:?}", labels, model);

    // Get all labels from the program fragment sorted in a topological order.
    let mut all_labels = Vec::new();
    for (_, block) in smt_context.vir.walk() {
        for statement in &block.statements {
            if let Some(label) = statement.get_label() {
                all_labels.push(smt_context.context.convert_known_label_name_to_smt(label));
            }
        }
    }
    eprintln!("all labels: {:?}", all_labels);
    let label_ids: HashMap<_, _> = all_labels
        .into_iter()
        .enumerate()
        .map(|(id, label)| (label, id))
        .collect();

    fn find_last_label<'a>(
        label_ids: &HashMap<smt::LabelSymbol, usize>,
        labels: impl Iterator<Item = &'a smt::LabelSymbol>,
    ) -> smt::LabelSymbol {
        labels.max_by_key(|label| label_ids[label]).unwrap().clone()
    }

    fn compute_failing_trace(
        label_ids: &HashMap<smt::LabelSymbol, usize>,
        labels: &[smt::LabelSymbol],
        failing_assertion: &smt::LabelSymbol,
    ) -> Vec<smt::LabelSymbol> {
        let mut trace: Vec<_> = labels
            .iter()
            .filter(|&label| label != failing_assertion)
            .cloned()
            .collect();
        trace.sort_by_cached_key(|label| label_ids[label]);
        trace
    }

    let failing_assertion = find_last_label(&label_ids, labels.iter());
    let failing_trace = compute_failing_trace(&label_ids, &labels, &failing_assertion);
    let mut errors = vec![VerificationError::new(
        smt_context,
        failing_assertion,
        failing_trace,
        &model,
    )];

    // The traces that we already explored in our search.
    let mut explored_traces = HashSet::new();
    fn to_trace_signature<'a>(
        label_ids: &HashMap<smt::LabelSymbol, usize>,
        labels: impl Iterator<Item = (&'a smt::LabelSymbol, bool)>,
    ) -> Vec<(usize, bool)> {
        let mut vec: Vec<_> = labels
            .map(|(label, value)| (label_ids[label], value))
            .collect();
        vec.sort_unstable();
        vec
    }
    explored_traces.insert(to_trace_signature(
        &label_ids,
        labels.iter().map(|label| (label, model.get_label(label))),
    ));

    let mut working_set: HashSet<_> = labels.into_iter().collect();

    while let Some(last) = working_set
        .iter()
        .max_by_key(|label| label_ids[label])
        .cloned()
    {
        // Remove the last label from the set.
        working_set.remove(&last);

        // Construct a trace with a last label inverted.
        let mut trace: Vec<_> = working_set
            .iter()
            .map(|label| (label, model.get_label(label)))
            .collect();
        trace.push((&last, !model.get_label(&last)));

        // Check whether we already visited this trace.
        let signature = to_trace_signature(&label_ids, trace.iter().cloned());
        if !explored_traces.insert(signature) {
            // The trace is already explored.
            continue;
        }

        // Construct the assertions that mark the trace we want to explore.
        let assertions: Vec<_> = trace
            .iter()
            .map(|&(label, value)| {
                let variable = smt::Expression::variable(label.clone());
                if value {
                    variable
                } else {
                    smt::Expression::not(variable)
                }
            })
            .collect();

        eprintln!("new assertions: {:?}", assertions);

        // Query the solver.
        let result = investigator.check_with(&assertions, smt_context)?;
        eprintln!("result: {:?}", result);
        if result != SatResult::Unsat {
            // We got a new failing trace.
            let new_labels = investigator.get_labels(smt_context)?;
            let new_model = investigator.get_model(smt_context)?;
            eprintln!("new labels = {:?} model = {:?}", new_labels, new_model);

            // // Check the new failing trace is an extension of the requested trace.
            // TODO: This assertion does not hold and we also generate the same
            // errors multiple times for some reason.
            // for &(label, value) in &trace {
            //     assert!(
            //         new_labels.iter().any(|new_label| new_label == label),
            //         "{:?} is not an extension of {:?}",
            //         new_labels,
            //         trace
            //     );
            //     assert_eq!(
            //         new_model.get_label(label),
            //         value,
            //         "New trace should be an extension."
            //     );
            // }

            let failing_assertion = find_last_label(&label_ids, new_labels.iter());
            let failing_trace = compute_failing_trace(&label_ids, &new_labels, &failing_assertion);
            errors.push(VerificationError::new(
                smt_context,
                failing_assertion,
                failing_trace,
                &new_model,
            ));

            working_set = new_labels.into_iter().collect();
            model = new_model;
        }
    }

    Ok(errors)
}
