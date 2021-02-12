use super::expression::lower_expression;
use crate::context::Context;
use crate::errors::SvirptiResult;
use index_vec::IndexVec;
use std::collections::{hash_map::Entry, HashMap};
use svirpti_vir::{
    common::{
        cfg::Cfg,
        expression::{BinaryOperationHelpers, SyntacticEvaluation, VariableHelpers},
        statement::AssumeAssertHelpers,
    },
    high, low,
};

pub fn lower_program<C: Context>(
    program: &high::ProgramFragment,
    context: &mut C,
) -> SvirptiResult<low::ProgramFragment> {
    // TODO: program.validate()?;
    program.procedure.validate();
    let mut basic_blocks = IndexVec::new();
    let predecessors = program.procedure.compute_predecessors();
    let mut variable_versions_after_block: HashMap<
        high::BasicBlockId,
        HashMap<high::VariableSymbol, usize>,
    > = HashMap::new();
    let mut all_variables: Vec<_> = program
        .procedure
        .variables
        .iter()
        .map(|variable| low::VariableDeclaration {
            name: context.create_versioned_variable_symbol(&variable.name, 0),
            sort: lower_type(context, &variable.sort),
        })
        .collect();
    let variable_sorts: HashMap<_, _> = program
        .procedure
        .variables
        .iter()
        .map(|variable| (variable.name.clone(), lower_type(context, &variable.sort)))
        .collect();
    let mut variable_counters: HashMap<_, _> = program
        .procedure
        .variables
        .iter()
        .map(|variable| (variable.name.clone(), 0))
        .collect();
    for (id, block) in program.procedure.walk() {
        eprintln!("walking: id={:?} block={:?}", id, block);
        let mut statements: IndexVec<_, low::Statement> = IndexVec::new();
        let predecessor_blocks = &predecessors[id];
        let mut variables: HashMap<_, _> = if predecessor_blocks.is_empty() {
            // Only the entry block has no predecessors. Initialize all
            // variables with version 0.
            program
                .procedure
                .variables
                .iter()
                .map(|variable| (variable.name.clone(), 0))
                .collect()
        } else if predecessor_blocks.len() == 1 {
            // Optimization: we have only one predecessor, use its variables.
            variable_versions_after_block[&predecessor_blocks[0]].clone()
        } else {
            // Merge the versions of variables in predecessors and emit the
            // equalities.
            let predecessor_variables: Vec<_> = predecessor_blocks
                .iter()
                .map(|id| &variable_versions_after_block[id])
                .collect();
            let mut merged_variables = HashMap::new();
            for variable in program.procedure.variables.iter() {
                let max_version = predecessor_variables
                    .iter()
                    .map(|map| map[&variable.name])
                    .max()
                    .unwrap();
                for map in &predecessor_variables {
                    let version = map[&variable.name];
                    if version != max_version {
                        statements.push(assume_var_eq(
                            context,
                            &variable.name,
                            version,
                            max_version,
                        ));
                    }
                }
                merged_variables.insert(variable.name.clone(), max_version);
            }
            merged_variables
        };

        if !block.guard.is_true() {
            let lowered_guard = lower_expression(context, &variables, &block.guard)?;
            statements.push(low::Statement::assume_with_label(
                lowered_guard,
                context.lower_label(&block.label),
            ));
        }

        for high_statement in &block.statements {
            eprintln!("high_statement: {:?}", high_statement);
            match high_statement {
                high::Statement::Assert(statement) => {
                    let label = statement
                        .label
                        .as_ref()
                        .map(|label| context.lower_label(label));
                    let assertion = lower_expression(context, &variables, &statement.assertion)?;
                    statements.push(low::Statement::Assert(low::Assert { assertion, label }));
                }
                high::Statement::Assume(statement) => {
                    let label = statement
                        .label
                        .as_ref()
                        .map(|label| context.lower_label(label));
                    let assertion = lower_expression(context, &variables, &statement.assertion)?;
                    statements.push(low::Statement::Assume(low::Assume { assertion, label }));
                }
                high::Statement::Havoc(_statement) => {
                    unimplemented!("havoc");
                }
                high::Statement::Assign(statement) => {
                    let variable = inc_var_version(
                        context,
                        &statement.variable,
                        &variable_sorts,
                        &mut variable_counters,
                        &mut all_variables,
                        &mut variables,
                    )?;
                    let expression = lower_expression(context, &variables, &statement.expression)?;
                    statements.push(low::Statement::assume(low::Expression::equals(
                        variable.into(),
                        expression,
                    )));
                }
            }
        }

        let successors = block
            .successors
            .iter()
            .map(|id| id.index().into())
            .collect();
        basic_blocks.push(low::BasicBlock {
            statements,
            successors,
        });
        variable_versions_after_block.insert(id, variables);
    }
    Ok(low::ProgramFragment {
        uninterpreted_sorts: Vec::new(), // TODO
        variables: all_variables,
        functions: Vec::new(), // TODO
        axioms: Vec::new(),    // TODO
        basic_blocks,
    })
}

fn inc_var_version<C: Context>(
    context: &mut C,
    variable: &high::Variable,
    variable_sorts: &HashMap<high::VariableSymbol, low::Sort>,
    variable_counters: &mut HashMap<high::VariableSymbol, usize>,
    all_variables: &mut Vec<low::VariableDeclaration>,
    variable_versions: &mut HashMap<high::VariableSymbol, usize>,
) -> SvirptiResult<low::Variable> {
    match variable_counters.entry(variable.name.clone()) {
        Entry::Occupied(mut entry) => {
            *entry.get_mut() += 1;
            let name = context.create_versioned_variable_symbol(&variable.name, *entry.get());
            if let Some(version) = variable_versions.get_mut(&variable.name) {
                *version = *entry.get();
            } else {
                unimplemented!("Report error: unknown variable {:?}", variable);
            }
            all_variables.push(low::VariableDeclaration {
                name: name.clone(),
                sort: variable_sorts[&variable.name].clone(), // TODO: Report error instead of panicking.
            });
            Ok(low::Variable { name })
        }
        Entry::Vacant(entry) => {
            unimplemented!("Report error: {:?}", entry);
        }
    }
}

fn assume_var_eq<C: Context>(
    context: &mut C,
    name: &high::VariableSymbol,
    version1: usize,
    version2: usize,
) -> low::Statement {
    low::Statement::assume(low::Expression::equals(
        low::Expression::variable(context.create_versioned_variable_symbol(name, version1)),
        low::Expression::variable(context.create_versioned_variable_symbol(name, version2)),
    ))
}

fn lower_type<C: Context>(context: &mut C, typ: &high::Type) -> low::Sort {
    match typ {
        high::Type::Int => low::Sort::Int,
        high::Type::Bool => low::Sort::Bool,
        high::Type::Real => low::Sort::Real,
        high::Type::Domain(high::DomainType { name }) => low::Sort::Uninterpreted {
            name: context.lower_domain_name(name),
        },
    }
}
