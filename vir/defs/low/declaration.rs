use super::context::*;

vir_include! { declaration =>
    use UninterpretedSortDeclaration;
    use VariableDeclaration;
    use FunctionDeclaration;
    use AxiomDeclaration;
    derive PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize;
}
