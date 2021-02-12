use super::context::*;
use super::expression::Variable;

vir_include! { statement =>
    use Assert;
    use Assume;
    use Havoc;
    use Assign;
    derive PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize;
}
vir_include! { statement::helpers =>
    use AssumeAssertHelpers;
}
pub use crate::common::statement::AssumeAssertHelpers;
vir_include! { statement::display =>
    use Assert;
    use Assume;
    use Havoc;
    use Assign;
}

#[derive(PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Statement {
    Assume(Assume),
    Assert(Assert),
    Havoc(Havoc),
    Assign(Assign),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Assert(statement) => statement.fmt(f),
            Statement::Assume(statement) => statement.fmt(f),
            Statement::Havoc(statement) => statement.fmt(f),
            Statement::Assign(statement) => statement.fmt(f),
        }
    }
}