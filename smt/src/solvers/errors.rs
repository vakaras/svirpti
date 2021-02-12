use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SmtSolverError {
    #[error("a generic SMT solver error")]
    GenericSolverError {
        #[from]
        source: Box<dyn std::error::Error>,
    },
}
