use crate::context::Context;
use std::collections::HashMap;
use svirpti_vir::{low, smt};

pub(crate) struct SmtContext<'a, C: Context> {
    pub(crate) context: &'a C,
    pub(crate) vir: low::ProgramFragment,
    pub(crate) variable_sorts: HashMap<smt::VariableSymbol, smt::Sort>,
    pub(crate) function_sorts: HashMap<smt::FunctionSymbol, smt::Sort>,
}

impl<'c, C: Context> smt::Context for SmtContext<'c, C> {
    fn write_uninterpreted_sort_name<Writer: std::io::Write>(
        &self,
        writer: &mut Writer,
        symbol: &smt::UninterpretedSortSymbol,
    ) -> rsmt2::SmtRes<()> {
        write!(writer, "{}", symbol)?;
        Ok(())
    }
    fn write_variable_symbol<Writer: std::io::Write>(
        &self,
        writer: &mut Writer,
        symbol: &smt::VariableSymbol,
    ) -> rsmt2::SmtRes<()> {
        write!(writer, "{}", symbol)?;
        Ok(())
    }
    fn write_label_symbol<Writer: std::io::Write>(
        &self,
        writer: &mut Writer,
        symbol: &smt::VariableSymbol,
    ) -> rsmt2::SmtRes<()> {
        write!(writer, "{}", symbol)?;
        Ok(())
    }
    fn get_variable_sort<'a>(&'a self, variable: &'a smt::VariableSymbol) -> &'a smt::Sort {
        &self.variable_sorts[variable]
    }
    fn get_function_sort<'a>(&'a self, function: &'a smt::FunctionSymbol) -> &'a smt::Sort {
        &self.function_sorts[function]
    }
    fn resolve_ident(&self, ident: &str) -> rsmt2::SmtRes<smt::IdentSymbol> {
        Ok(ident.into())
    }
}
