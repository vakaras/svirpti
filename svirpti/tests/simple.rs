use svirpti::{context::StringContext, encode, lower, verify};
use svirpti_smt::solvers::z3_smt2::Z3SmtSolver;
use svirpti_vir::common::{expression::VariableHelpers, statement::AssumeAssertHelpers};
use svirpti_vir::high;
use svirpti_vir_derive::vir_high;

fn trivial_fail() -> high::ProgramFragment {
    high::ProgramFragment {
        sorts: vec![],
        axioms: vec![],
        functions: vec![],
        procedure: high::ProcedureDeclaration {
            variables: vec![high::VariableDeclaration {
                name: "x".into(),
                sort: high::Type::Bool,
            }]
            .into(),
            basic_blocks: vec![
                high::BasicBlock {
                    label: "entry".into(),
                    guard: true.into(),
                    statements: vec![].into(),
                    successors: vec![1.into()],
                },
                high::BasicBlock {
                    label: "bb1".into(),
                    guard: true.into(),
                    statements: vec![high::Statement::assert_with_label(
                        high::Expression::variable("x".into()),
                        "expected_error".into(),
                    )]
                    .into(),
                    successors: vec![2.into()],
                },
                high::BasicBlock {
                    label: "exit".into(),
                    guard: true.into(),
                    statements: vec![].into(),
                    successors: vec![],
                },
            ]
            .into(),
        },
    }
}

#[test]
fn trivial_fail_macro_test() {
    let program_macro = vir_high! {
        procedure {
            locals {
                x: Bool,
            }
            bb1 {
                guard true;
                assert expected_error x;
                goto { exit }
            }
        }
    };
    let program_manual = trivial_fail();
    assert_eq!(program_macro, program_manual);
}

#[test]
fn lower_trivial_fail() {
    let program = trivial_fail();
    insta::assert_yaml_snapshot!(program);
    let mut context = StringContext {};
    let lowered = lower(&program, &mut context).unwrap();
    insta::assert_yaml_snapshot!(lowered);
}

#[test]
fn encode_trivial_fail() {
    let program = trivial_fail();
    let mut context = StringContext {};
    let lowered = lower(&program, &mut context).unwrap();
    let encoded = encode(&lowered, &mut context).unwrap();
    insta::assert_yaml_snapshot!(encoded);
}

#[test]
fn verify_trivial_fail() {
    let program = trivial_fail();
    let mut context = StringContext {};
    match verify::<Z3SmtSolver, _>(&mut context, &program).unwrap() {
        svirpti::VerificationResult::Failure(failure) => {
            let errors = failure.get_all_errors().unwrap();
            insta::assert_yaml_snapshot!(errors);
        }
        x => unreachable!("{:?}", x),
    }
}

#[test]
fn check_assume_encoding() {
    let program = vir_high! {
        procedure {
            locals {
                a: Bool,
                b: Bool,
            }
            bb1 {
                guard !a;
                assume l1 a;
                assert l2 b;
                goto { exit }
            }
        }
    };
    insta::assert_yaml_snapshot!(program);
    let mut context = StringContext {};
    insta::assert_yaml_snapshot!(lower(&program, &mut context).unwrap());
    let lowered = lower(&program, &mut context).unwrap();
    insta::assert_display_snapshot!(lowered);
    let encoded = encode(&lowered, &mut context).unwrap();
    insta::assert_display_snapshot!(encoded);
    assert!(verify::<Z3SmtSolver, _>(&mut context, &program)
        .unwrap()
        .is_success());
}

#[test]
fn check_sequence_encoding() {
    let program = vir_high! {
        procedure {
            locals {
                a: Int,
            }
            bb1 {
                guard 0 < a;
                goto { bb2 }
            }
            bb2 {
                guard a < 10;
                goto { bb3 }
            }
            bb3 {
                guard true;
                assert l 0 < a && a < 10;
                goto { exit }
            }
        }
    };
    insta::assert_yaml_snapshot!(program);
    let mut context = StringContext {};
    insta::assert_yaml_snapshot!(lower(&program, &mut context).unwrap());
    let lowered = lower(&program, &mut context).unwrap();
    insta::assert_display_snapshot!(lowered);
    let encoded = encode(&lowered, &mut context).unwrap();
    insta::assert_display_snapshot!(encoded);
    assert!(verify::<Z3SmtSolver, _>(&mut context, &program)
        .unwrap()
        .is_success());
}

#[test]
fn check_non_deterministic_choice_encoding() {
    let program = vir_high! {
        procedure {
            locals {
                a: Bool,
                b: Bool,
                x: Int,
            }
            bb1 {
                guard true;
                goto { bb2, bb3 }
            }
            bb2 {
                guard a;
                assume l1 x > 5;
                goto { bb4 }
            }
            bb3 {
                guard b;
                assume l2 x < 8;
                goto { bb4 }
            }
            bb4 {
                guard true;
                assert l3 (a -> x > 5) || (b -> x < 8);
                goto { exit }
            }
        }
    };
    insta::assert_yaml_snapshot!(program);
    let mut context = StringContext {};
    insta::assert_yaml_snapshot!(lower(&program, &mut context).unwrap());
    let lowered = lower(&program, &mut context).unwrap();
    insta::assert_display_snapshot!(lowered);
    let encoded = encode(&lowered, &mut context).unwrap();
    insta::assert_display_snapshot!(encoded);
    assert!(verify::<Z3SmtSolver, _>(&mut context, &program)
        .unwrap()
        .is_success());
}

#[test]
fn check_non_deterministic_choice_encoding2() {
    let program = vir_high! {
        procedure {
            locals {
                a: Bool,
                b: Bool,
                c: Bool,
            }
            bb1 {
                guard true;
                goto { bb2, bb3 }
            }
            bb2 {
                guard a;
                assume l1 b;
                goto { bb4 }
            }
            bb3 {
                guard c;
                goto { bb4 }
            }
            bb4 {
                guard true;
                assert l3 (a -> b) || c;
                goto { exit }
            }
        }
    };
    insta::assert_yaml_snapshot!(program);
    let mut context = StringContext {};
    insta::assert_yaml_snapshot!(lower(&program, &mut context).unwrap());
    let lowered = lower(&program, &mut context).unwrap();
    insta::assert_display_snapshot!(lowered);
    let encoded = encode(&lowered, &mut context).unwrap();
    insta::assert_display_snapshot!(encoded);
    assert!(verify::<Z3SmtSolver, _>(&mut context, &program)
        .unwrap()
        .is_success());
}

#[test]
fn check_assign_encoding() {
    let program = vir_high! {
        procedure {
            locals {
                a: Int,
                b: Int,
            }
            bb1 {
                guard true;
                assign a = b;
                goto { bb2 }
            }
            bb2 {
                guard true;
                assert l1 a == b;
                goto { exit }
            }
        }
    };
    insta::assert_yaml_snapshot!(program);
    let mut context = StringContext {};
    insta::assert_yaml_snapshot!(lower(&program, &mut context).unwrap());
    let lowered = lower(&program, &mut context).unwrap();
    insta::assert_display_snapshot!(lowered);
    let encoded = encode(&lowered, &mut context).unwrap();
    insta::assert_display_snapshot!(encoded);
    assert!(verify::<Z3SmtSolver, _>(&mut context, &program)
        .unwrap()
        .is_success());
}

#[test]
fn check_multiple_errors() {
    let program = vir_high! {
        procedure {
            locals {
                x: Int,
            }
            bb1 {
                guard true;
                goto { bb2, bb3 }
            }
            bb2 {
                guard x > 0;
                assert l1 x == 2;
                goto { bb4 }
            }
            bb3 {
                guard !(x > 0);
                assert l2 x < 0;
                assert l3 x != 0;   // TODO: This error should not be reported.
                goto { bb4 }
            }
            bb4 {
                guard true;
                goto { exit }
            }
        }
    };
    insta::assert_display_snapshot!(program);
    let mut context = StringContext {};
    let lowered = lower(&program, &mut context).unwrap();
    insta::assert_display_snapshot!(lowered);
    let encoded = encode(&lowered, &mut context).unwrap();
    insta::assert_display_snapshot!(encoded);
    match verify::<Z3SmtSolver, _>(&mut context, &program).unwrap() {
        svirpti::VerificationResult::Failure(failure) => {
            let errors = failure.get_all_errors().unwrap();
            insta::assert_yaml_snapshot!(errors);
        }
        x => unreachable!("{:?}", x),
    }
}
