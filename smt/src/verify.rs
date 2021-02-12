use crate::solvers::{self, SatResult, SmtSolver};
use solvers::SmtSolverError;
use svirpti_vir::smt as ast;
use thiserror::Error;

#[derive(Debug)]
pub enum VerificationResult<S: SmtSolver> {
    Success,
    Failure(VerificationFailure<S>),
}

#[derive(Debug, Error)]
pub enum VerifierError {
    #[error("smt solver error")]
    SmtSolverError(#[from] SmtSolverError),
}

pub type VerifierResult<T = ()> = Result<T, VerifierError>;

pub fn verify<S: SmtSolver, C: ast::Context>(
    context: &C,
    declarations: &ast::Declarations,
    assertions: &[ast::Expression],
) -> VerifierResult<VerificationResult<S>> {
    let solver = S::default()?;
    verify_with_solver(solver, context, declarations, assertions)
}

#[derive(Debug)]
pub struct VerificationFailure<S: SmtSolver> {
    investigator: VerificationFailureInvestigator<S>,
}

impl<S: SmtSolver> VerificationFailure<S> {
    pub fn get_investigator(&mut self) -> &mut VerificationFailureInvestigator<S> {
        &mut self.investigator
    }
}

/// A struct that allows investigating a verification failure by checking the
/// state with different labels and obtaining models.
pub struct VerificationFailureInvestigator<S: SmtSolver> {
    solver: S,
    labels: Option<Vec<ast::LabelSymbol>>,
    model: Option<ast::Model>,
}

impl<S: SmtSolver> std::fmt::Debug for VerificationFailureInvestigator<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VerificationFailureInvestigator")
    }
}

impl<S: SmtSolver> VerificationFailureInvestigator<S> {
    fn new(solver: S) -> Self {
        Self {
            solver,
            labels: None,
            model: None,
        }
    }
    pub fn check_with(
        &mut self,
        assertions: &[ast::Expression],
        context: &impl ast::Context,
    ) -> VerifierResult<SatResult> {
        self.solver.push()?;
        for assertion in assertions {
            self.solver.assert(assertion, context)?;
        }
        let result = self.solver.check_sat()?;
        if result != SatResult::Unsat {
            self.labels = Some(self.solver.get_labels(context)?);
            self.model = Some(self.solver.get_model(context)?);
        }
        self.solver.pop()?;
        Ok(result)
    }
    pub fn get_labels(
        &mut self,
        context: &impl ast::Context,
    ) -> VerifierResult<Vec<ast::LabelSymbol>> {
        if let Some(labels) = self.labels.take() {
            Ok(labels)
        } else {
            Ok(self.solver.get_labels(context)?)
        }
    }
    pub fn get_model(&mut self, context: &impl ast::Context) -> VerifierResult<ast::Model> {
        if let Some(model) = self.model.take() {
            Ok(model)
        } else {
            Ok(self.solver.get_model(context)?)
        }
    }
}

fn verify_with_solver<S: SmtSolver, C: ast::Context>(
    mut solver: S,
    context: &C,
    declarations: &ast::Declarations,
    assertions: &[ast::Expression],
) -> VerifierResult<VerificationResult<S>> {
    for sort in &declarations.sorts {
        solver.declare_sort(sort, context)?;
    }
    for function in &declarations.functions {
        solver.declare_function(function, context)?;
    }
    for label in &declarations.labels {
        solver.declare_label(label, context)?;
    }
    for variable in &declarations.variables {
        solver.declare_variable(variable, context)?;
    }
    solver.push()?;
    for assertion in assertions {
        solver.assert(assertion, context)?;
    }
    let sat_result = solver.check_sat()?;
    if sat_result == SatResult::Unsat {
        Ok(VerificationResult::Success)
    } else {
        let investigator = VerificationFailureInvestigator::new(solver);
        Ok(VerificationResult::Failure(VerificationFailure {
            investigator,
        }))
    }
}
