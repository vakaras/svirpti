use crate::wp;
use crate::{context::Context, lower::SmtContext, SvirptiResult};
use std::collections::BTreeMap;
use svirpti_smt::solvers::SmtSolver;
use svirpti_vir::{high, low, smt};

#[derive(PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Model {
    // TODO: Instead of `low::VariableSymbol`, use `high::VariableSymbol`
    // together with position information (`high::BasicBlockId`,
    // `high::StatementId`) where that version of the variable was introduced.
    variables: BTreeMap<low::VariableSymbol, smt::Value>,
}

impl Model {
    pub fn new<C: Context>(_context: &C, model: &wp::Model) -> Self {
        Self {
            variables: model.variables.clone(),
        }
    }
}
#[derive(PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerificationError {
    /// The assertion that failed.
    pub failing_assertion: high::LabelSymbol,
    /// The basic blocks that led to the failing assertion. (Note: it is a path
    /// that leads to the failing assertion; however, there could be also other
    /// paths that go through the same `labels`.)
    pub trace: Vec<high::BasicBlockId>,
    /// The labels reported by the SMT solver.
    pub labels: Vec<high::LabelSymbol>,
    /// The model returned by the SMT solver.
    pub model: Model,
}

impl VerificationError {
    pub fn get_trace_labels(
        &self,
        procedure: &high::ProcedureDeclaration,
    ) -> Vec<high::LabelSymbol> {
        self.trace
            .iter()
            .map(|&id| procedure.basic_blocks[id].label.clone())
            .collect()
    }
}

pub enum VerificationResult<'a, C: Context, S: SmtSolver> {
    Success,
    Failure(VerificationFailure<'a, C, S>),
}

impl<'a, C: Context, S: SmtSolver> VerificationResult<'a, C, S> {
    pub fn is_success(&self) -> bool {
        matches!(self, VerificationResult::Success)
    }
}

impl<'a, C: Context, S: SmtSolver> std::fmt::Debug for VerificationResult<'a, C, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationResult::Success => write!(f, "Success"),
            VerificationResult::Failure(_) => write!(f, "Failure"),
        }
    }
}

pub struct VerificationFailure<'a, C: Context, S: SmtSolver> {
    pub(crate) smt_context: SmtContext<'a, C>,
    pub(crate) smt_failure: svirpti_smt::VerificationFailure<S>,
}

impl<'a, C: Context, S: SmtSolver> VerificationFailure<'a, C, S> {
    pub fn get_all_errors(self) -> SvirptiResult<Vec<VerificationError>> {
        let mut errors = Vec::new();
        let context = self.smt_context.context;
        for error in crate::wp::get_all_errors(self.smt_failure, &self.smt_context)? {
            eprintln!("error: {:?}", error);
            errors.push(VerificationError {
                failing_assertion: context.resolve_high_label(&error.failing_assertion),
                labels: error
                    .labels
                    .iter()
                    .map(|label| context.resolve_high_label(label))
                    .collect(),
                trace: error.trace.iter().map(|id| id.index().into()).collect(),
                model: Model::new(context, &error.model),
            });
        }
        Ok(errors)
    }
}
