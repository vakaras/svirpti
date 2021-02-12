pub trait Context {}

pub type Sort = super::sort::Sort;
pub type Expression = super::expression::Expression;
pub type Statement = super::statement::Statement;
crate::derive_string_symbol!(UninterpretedSortSymbol);
crate::derive_string_symbol!(VariableSymbol);
crate::derive_string_symbol!(FunctionSymbol);
crate::derive_string_symbol!(AxiomNameSymbol);
crate::derive_string_symbol!(LabelSymbol);
