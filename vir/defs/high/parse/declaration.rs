vir_include! { declaration::parse_ast =>
    use UninterpretedSortDeclaration;
    use VariableDeclaration;
    use FunctionDeclaration;
    use AxiomDeclaration;
    derive PartialEq, Eq, Debug, Clone;
}
vir_include! { declaration::parse =>
    use kw;
    use UninterpretedSortDeclaration;
    use VariableDeclaration;
    use FunctionDeclaration;
    use AxiomDeclaration;
}

pub use super::expression::Expression;
pub use super::sort::Sort;
