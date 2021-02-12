#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct IdentSymbol(String);

impl IdentSymbol {
    pub fn as_string(&self) -> String {
        self.0.clone()
    }
}

impl From<&str> for IdentSymbol {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<String> for IdentSymbol {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for IdentSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a, C: super::context::Context> rsmt2::print::Sym2Smt<&'a C> for IdentSymbol {
    fn sym_to_smt2<Writer: std::io::Write>(
        &self,
        writer: &mut Writer,
        _context: &'a C,
    ) -> rsmt2::SmtRes<()> {
        write!(writer, "{}", self.0)?;
        Ok(())
    }
}

impl<'a, C: super::context::Context> ::rsmt2::print::Expr2Smt<&'a C> for IdentSymbol {
    fn expr_to_smt2<Writer: std::io::Write>(
        &self,
        writer: &mut Writer,
        _context: &'a C,
    ) -> ::rsmt2::SmtRes<()> {
        write!(writer, "{}", self.0)?;
        Ok(())
    }
}
