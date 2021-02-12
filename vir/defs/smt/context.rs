pub trait Context {
    fn write_uninterpreted_sort_name<Writer: std::io::Write>(
        &self,
        writer: &mut Writer,
        symbol: &UninterpretedSortSymbol,
    ) -> rsmt2::SmtRes<()>;
    fn write_variable_symbol<Writer: std::io::Write>(
        &self,
        writer: &mut Writer,
        symbol: &VariableSymbol,
    ) -> rsmt2::SmtRes<()>;
    fn write_label_symbol<Writer: std::io::Write>(
        &self,
        writer: &mut Writer,
        symbol: &LabelSymbol,
    ) -> rsmt2::SmtRes<()>;
    fn get_variable_sort<'a>(&'a self, variable: &'a VariableSymbol) -> &'a Sort;
    fn get_function_sort<'a>(&'a self, function: &'a FunctionSymbol) -> &'a Sort;
    fn resolve_ident(&self, ident: &str) -> rsmt2::SmtRes<IdentSymbol>;
}

pub use super::ident::IdentSymbol;
pub type Sort = super::sort::Sort;
pub type Expression = super::expression::Expression;
pub type UninterpretedSortSymbol = IdentSymbol;
pub type UninterpretedSortValue = String;
pub type VariableSymbol = IdentSymbol;
pub type LabelSymbol = IdentSymbol;
pub type FunctionSymbol = IdentSymbol;
