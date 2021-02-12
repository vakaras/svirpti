use super::declaration::Declarations;
use super::expression::Expression;

pub type Assertions = Vec<Expression>;

#[derive(PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Query {
    pub declarations: Declarations,
    pub assertions: Assertions,
}

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "program {{")?;
        writeln!(f, "  declarations:")?;
        writeln!(f, "    sorts:")?;
        for sort in &self.declarations.sorts {
            writeln!(f, "      {}", sort)?;
        }
        writeln!(f, "    variables:")?;
        for variable in &self.declarations.variables {
            writeln!(f, "      {}", variable)?;
        }
        writeln!(f, "    functions:")?;
        for function in &self.declarations.functions {
            writeln!(f, "      {}", function)?;
        }
        writeln!(f, "    labels:")?;
        for label in &self.declarations.labels {
            writeln!(f, "      {}", label)?;
        }
        writeln!(f, "  assertions:")?;
        for assertion in &self.assertions {
            writeln!(f, "    {}", assertion)?;
        }
        writeln!(f, "}}")
    }
}
