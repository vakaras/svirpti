//! Note: it seems that the text based smt2 interface is the only one that
//! supports labels. (Z3, CVC4, and other solvers do not have an API for using
//! labels.)

pub use self::errors::SmtSolverError;
use svirpti_vir::smt as ast;

pub mod errors;
pub mod z3_smt2;

pub enum SatisfiabilityResult<Model> {
    Unsat,
    Unknown(Model),
    Sat(Model),
}

#[derive(Debug, PartialEq, Eq)]
pub enum SatResult {
    Unsat,
    Unknown,
    Sat,
}

pub type SmtSolverResult<T = ()> = Result<T, SmtSolverError>;

pub trait SmtSolver: Sized {
    type Conf;
    type Error: std::fmt::Debug;
    fn default() -> SmtSolverResult<Self>;
    fn new(conf: Self::Conf) -> SmtSolverResult<Self>;
    fn declare_sort(
        &mut self,
        sort: &ast::UninterpretedSortDeclaration,
        context: &impl ast::Context,
    ) -> SmtSolverResult;
    fn declare_function(
        &mut self,
        function: &ast::FunctionDeclaration,
        context: &impl ast::Context,
    ) -> SmtSolverResult;
    fn declare_label(
        &mut self,
        label: &ast::LabelDeclaration,
        context: &impl ast::Context,
    ) -> SmtSolverResult;
    fn declare_variable(
        &mut self,
        variable: &ast::VariableDeclaration,
        context: &impl ast::Context,
    ) -> SmtSolverResult;
    fn push(&mut self) -> SmtSolverResult;
    fn pop(&mut self) -> SmtSolverResult;
    fn assert(
        &mut self,
        assertion: &ast::Expression,
        context: &impl ast::Context,
    ) -> SmtSolverResult;
    fn check_sat(&mut self) -> SmtSolverResult<SatResult>;
    fn get_labels(&mut self, context: &impl ast::Context)
        -> SmtSolverResult<Vec<ast::LabelSymbol>>;
    fn get_model(&mut self, context: &impl ast::Context) -> SmtSolverResult<ast::Model>;
}
