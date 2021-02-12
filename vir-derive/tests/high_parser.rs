use svirpti_vir_derive::vir_high;

#[test]
fn unary_expression() {
    let program = vir_high! {
        procedure {
            locals {
                a: Bool,
            }
            bb1 {
                guard !a && true;
                goto { exit }
            }
        }
    };
    insta::assert_yaml_snapshot!(program);
    insta::assert_display_snapshot!(program);
}

#[test]
fn unary_expression2() {
    let program = vir_high! {
        procedure {
            locals {
                a: Bool,
            }
            bb1 {
                guard true && !a;
                assume label1 !a && !a;
                assume label1 !a && !a && !!!!a;
                goto { exit }
            }
        }
    };
    insta::assert_display_snapshot!(program);
    insta::assert_yaml_snapshot!(program);
}

#[test]
fn binary_operation() {
    let program = vir_high! {
        procedure {
            locals {
                a: Bool,
                b: Int,
            }
            bb1 {
                guard true;
                assume label1 a && a && a;
                assume label2 a || a || a;
                assume label3 a && a || a;
                assume label3 a || a && a;
                assume label4 b > 0 && a;
                assume label5 a && b > 0;
                goto { exit }
            }
        }
    };
    insta::assert_display_snapshot!(program);
    insta::assert_yaml_snapshot!(program);
}

#[test]
fn implies() {
    let program = vir_high! {
        procedure {
            locals {
                a: Bool,
            }
            bb1 {
                guard true -> false;
                assume label1 a && a && !a -> !a && a || a;
                assume label2 a && !a || a -> a;
                goto { exit }
            }
        }
    };
    insta::assert_display_snapshot!(program);
    insta::assert_yaml_snapshot!(program);
}

#[test]
fn quantifiers() {
    let program = vir_high! {
        procedure {
            locals {
                a: Bool,
            }
            bb1 {
                guard forall(|b: Int| a > 5 -> a > 4 && true, []);
                assume label1 forall(|a: Int| a > 5 -> a > 4 && true, [(a, true)]);
                goto { exit }
            }
        }
    };
    insta::assert_display_snapshot!(program);
    insta::assert_yaml_snapshot!(program);
}
