use super::expression;
use crate::context::Context;
use crate::errors::SvirptiResult;
use std::collections::HashMap;
use svirpti_vir::common::cfg::Cfg;
use svirpti_vir::common::expression::{
    BinaryOperationHelpers, ExpressionIterator, LabelledExpressionHelpers, UnaryOperationHelpers,
    VariableHelpers,
};
use svirpti_vir::{low, smt};

fn convert_sort_to_smt<C: Context>(context: &mut C, sort: &low::Sort) -> smt::Sort {
    match sort {
        low::Sort::Bool => smt::Sort::Bool,
        low::Sort::Int => smt::Sort::Int,
        low::Sort::Real => smt::Sort::Real,
        low::Sort::Uninterpreted { name } => smt::Sort::Uninterpreted {
            name: context.convert_uninterpreted_sort_to_smt(name),
        },
    }
}

pub fn encode<C: Context>(
    program: &low::ProgramFragment,
    context: &mut C,
) -> SvirptiResult<smt::Query> {
    // TODO: program.validate();
    let mut assertions = Vec::new();
    let mut variables: Vec<_> = program
        .variables
        .iter()
        .map(|variable| smt::VariableDeclaration {
            name: context.convert_variable_name_to_smt(&variable.name),
            sort: convert_sort_to_smt(context, &variable.sort),
        })
        .collect();
    let mut labels = Vec::new();
    let mut basic_block_wps: HashMap<low::BasicBlockId, smt::Expression> = HashMap::new();
    for (id, block) in program.reverse_walk() {
        let mut wp = block
            .successors
            .iter()
            .map(|successor| basic_block_wps[successor].clone())
            .conjoin();
        for statement in block.statements.iter().rev() {
            eprintln!("     current wp: {}", wp);
            match statement {
                low::Statement::Assert(low::Assert { assertion, label }) => {
                    if let Some(label) = label {
                        let name = context.convert_label_name_to_smt(&label);
                        labels.push(smt::LabelDeclaration { name: name.clone() });
                        let conjunct = smt::Expression::label_negative(
                            name,
                            lower_expression(context, assertion)?,
                        );
                        wp = smt::Expression::and(conjunct, wp);
                    } else {
                        wp = smt::Expression::and(lower_expression(context, assertion)?, wp);
                    }
                }
                low::Statement::Assume(low::Assume { assertion, label }) => {
                    if let Some(label) = label {
                        let name = context.convert_label_name_to_smt(&label);
                        labels.push(smt::LabelDeclaration { name: name.clone() });
                        let condition = smt::Expression::label_positive(
                            name,
                            lower_expression(context, assertion)?,
                        );
                        wp = smt::Expression::implies(condition, wp);
                    } else {
                        wp = smt::Expression::implies(lower_expression(context, assertion)?, wp);
                    }
                }
            }
        }
        let basic_block_label = context.create_label_for_basic_block(id);
        variables.push(smt::VariableDeclaration {
            name: basic_block_label.clone(),
            sort: smt::Sort::Bool,
        });
        let basic_block_label_expression = smt::Expression::variable(basic_block_label);
        basic_block_wps.insert(id, basic_block_label_expression.clone());
        eprintln!("block {:?}:  {} {}", id, basic_block_label_expression, wp);
        assertions.push(smt::Expression::equals(basic_block_label_expression, wp));
    }
    assertions.push(smt::Expression::not(smt::Expression::variable(
        context.create_label_for_basic_block(0.into()),
    )));
    let declarations = smt::Declarations {
        sorts: Vec::new(),
        functions: Vec::new(),
        labels,
        variables,
    };
    Ok(smt::Query {
        declarations,
        assertions,
    })
}

struct ExpressionLowerer<'a, C: Context> {
    context: &'a mut C,
}

impl<'a, C: Context> expression::Lowerer for ExpressionLowerer<'a, C> {
    fn lower_variable_symbol(
        &mut self,
        symbol: &low::VariableSymbol,
    ) -> SvirptiResult<smt::VariableSymbol> {
        Ok(self.context.convert_variable_name_to_smt(symbol))
    }
    fn lower_function_symbol(
        &mut self,
        symbol: &low::FunctionSymbol,
    ) -> SvirptiResult<smt::FunctionSymbol> {
        Ok(self.context.convert_function_name_to_smt(symbol))
    }
    fn lower_sort_symbol(
        &mut self,
        symbol: &low::UninterpretedSortSymbol,
    ) -> SvirptiResult<smt::UninterpretedSortSymbol> {
        Ok(self.context.convert_uninterpreted_sort_to_smt(symbol))
    }
}

fn lower_expression<C: Context>(
    context: &mut C,
    expression: &low::Expression,
) -> SvirptiResult<smt::Expression> {
    let mut lowerer = ExpressionLowerer { context };
    expression::Lowerable::lower(expression, &mut lowerer)
}
