use super::context::*;

vir_include! { statement =>
    use Assert;
    use Assume;
    derive PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize;
}
vir_include! { statement::helpers =>
    use AssumeAssertHelpers;
}
pub use crate::common::statement::AssumeAssertHelpers;
vir_include! { statement::display =>
    use Assert;
    use Assume;
}

#[derive(PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Statement {
    Assert(Assert),
    Assume(Assume),
}

impl Statement {
    pub fn get_label(&self) -> Option<&LabelSymbol> {
        match self {
            Statement::Assert(Assert { label, .. }) | Statement::Assume(Assume { label, .. }) => {
                label.as_ref()
            }
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Assert(statement) => statement.fmt(f),
            Statement::Assume(statement) => statement.fmt(f),
        }
    }
}