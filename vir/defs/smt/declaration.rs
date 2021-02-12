use super::context::*;

vir_include! { declaration =>
    use UninterpretedSortDeclaration;
    use VariableDeclaration;
    use FunctionDeclaration;
    use LabelDeclaration;
    derive PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize;
}

vir_include! { declaration::rsmt =>
    use UninterpretedSortDeclaration;
}

vir_include! { declaration::display =>
    use UninterpretedSortDeclaration;
    use VariableDeclaration;
    use FunctionDeclaration;
    use LabelDeclaration;
}

#[derive(PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Declarations {
    pub sorts: Vec<UninterpretedSortDeclaration>,
    pub variables: Vec<VariableDeclaration>,
    pub functions: Vec<FunctionDeclaration>,
    pub labels: Vec<LabelDeclaration>,
}
