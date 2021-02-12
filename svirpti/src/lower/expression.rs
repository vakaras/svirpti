use super::generic_expression::{Lowerable, Lowerer};
use crate::context::Context;
use crate::errors::SvirptiResult;
use std::collections::HashMap;
use svirpti_vir::{high, low};

pub(crate) fn lower_expression<C: Context>(
    context: &mut C,
    variable_versions: &HashMap<high::VariableSymbol, usize>,
    expression: &high::Expression,
) -> SvirptiResult<low::Expression> {
    let mut lowerer = ExpressionLowerer {
        context,
        variable_versions,
    };
    Lowerable::lower(expression, &mut lowerer)
}

struct ExpressionLowerer<'a, C: Context> {
    context: &'a mut C,
    variable_versions: &'a HashMap<high::VariableSymbol, usize>,
}

impl<'a, C: Context> Lowerer for ExpressionLowerer<'a, C> {
    fn lower_function_symbol(
        &mut self,
        symbol: &high::FunctionSymbol,
    ) -> SvirptiResult<low::FunctionSymbol> {
        Ok(symbol.as_string().into())
    }
    fn lower_variable_symbol(
        &mut self,
        symbol: &high::VariableSymbol,
    ) -> SvirptiResult<low::VariableSymbol> {
        let version = self.variable_versions[symbol];
        Ok(self
            .context
            .create_versioned_variable_symbol(symbol, version))
    }
    fn lower_sort_symbol(
        &mut self,
        symbol: &high::UninterpretedSortSymbol,
    ) -> SvirptiResult<low::UninterpretedSortSymbol> {
        Ok(symbol.as_string().into())
    }
}
