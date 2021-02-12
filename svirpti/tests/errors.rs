//! Examples from papers and tutorials to check that all errors are reported as
//! expected.

use svirpti::{context::StringContext, lower, verify};
use svirpti_smt::solvers::z3_smt2::Z3SmtSolver;
use svirpti_vir_derive::vir_high;

/// An example from
/// https://www.microsoft.com/en-us/research/wp-content/uploads/2016/12/krml120.pdf
#[test]
fn labels_paper_example() {
    let program = vir_high! {
        procedure {
            locals {
                j: Int,
                k: Int,
            }
            bb1 {
                guard true;
                goto { bb_then, bb_else }
            }
            bb_then {
                guard !(k < 10) && k < 20;
                assign j = k;
                goto { bb_merge }
            }
            bb_else {
                guard !(!(k < 10) && k < 20);
                assign j = k;
                goto { bb_merge }
            }
            bb_merge {
                guard true;
                assert bounds_check 0 <= j && j < 100;
                goto { exit }
            }
        }
    };
    let mut context = StringContext {};
    insta::assert_display_snapshot!(lower(&program, &mut context).unwrap());
    match verify::<Z3SmtSolver, _>(&mut context, &program).unwrap() {
        svirpti::VerificationResult::Failure(failure) => {
            let mut errors = failure.get_all_errors().unwrap();
            assert_eq!(errors.len(), 1);
            let error = errors.pop().unwrap();
            insta::assert_yaml_snapshot!((&error, error.get_trace_labels(&program.procedure)));
        }
        x => unreachable!("{:?}", x),
    }
}
