#[derive(Debug)]
pub struct SvirptiError {
    verifier_error: svirpti_smt::VerifierError,
}

impl From<svirpti_smt::VerifierError> for SvirptiError {
    fn from(error: svirpti_smt::VerifierError) -> Self {
        Self {
            verifier_error: error,
        }
    }
}

pub type SvirptiResult<T = ()> = Result<T, SvirptiError>;
