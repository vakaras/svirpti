use self::lower::SmtContext;
pub use errors::{SvirptiError, SvirptiResult};
use svirpti_smt::solvers::SmtSolver;
use svirpti_vir::{high, smt};

pub use self::context::Context;
pub use self::lower::lower;
pub use self::wp::encode;
pub use verification_result::{VerificationError, VerificationFailure, VerificationResult};

pub mod context;
mod errors;
mod lower;
mod verification_result;
mod wp;

pub fn verify<'a, S: SmtSolver, C: Context>(
    context: &'a mut C,
    program: &high::ProgramFragment,
) -> Result<VerificationResult<'a, C, S>, SvirptiError> {
    let lowered_vir = lower(&program, context)?;
    let smt::Query {
        declarations,
        assertions,
    } = encode(&lowered_vir, context)?;
    let smt_context = SmtContext {
        context,
        vir: lowered_vir,
        variable_sorts: declarations
            .variables
            .iter()
            .map(|variable| (variable.name.clone(), variable.sort.clone()))
            .collect(),
        function_sorts: declarations
            .functions
            .iter()
            .map(|function| (function.name.clone(), function.return_sort.clone()))
            .collect(),
    };
    let result = svirpti_smt::verify(&smt_context, &declarations, &assertions)?;
    let result = match result {
        svirpti_smt::VerificationResult::Success => VerificationResult::Success,
        svirpti_smt::VerificationResult::Failure(failure) => {
            VerificationResult::Failure(VerificationFailure {
                smt_context,
                smt_failure: failure,
            })
        }
    };
    Ok(result)
}
