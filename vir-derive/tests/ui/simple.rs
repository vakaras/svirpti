use svirpti_vir_derive::vir_high;
use svirpti_vir::high::ProgramFragment;

fn empty() {
    let _ = vir_high!{

    };
}

fn no_locals() {
    let _ = vir_high!{
        procedure { }
    };
}

fn minimal() {
    let _: ProgramFragment = vir_high!{
        procedure {
            locals {}
        }
    };
}

fn exit_basic_block() {
    let _: ProgramFragment = vir_high!{
        procedure {
            locals {}
            exit { }
        }
    };
}

fn single_basic_block() {
    let _: ProgramFragment = vir_high!{
        procedure {
            locals {}
            bb1 {
                guard true;
                goto { exit }
            }
        }
    };
}

fn missing_basic_block() {
    let _: ProgramFragment = vir_high!{
        procedure {
            locals {}
            bb1 {
                guard true;
                goto { bb2 }
            }
        }
    };
}

fn diamond_basic_block() {
    let _: ProgramFragment = vir_high!{
        procedure {
            locals {}
            bb1 {
                guard true;
                goto { bb2, bb3 }
            }
            bb2 {
                guard true;
                goto { bb4 }
            }
            bb3 {
                guard true;
                goto { bb4 }
            }
            bb4 {
                guard true;
                goto { exit }
            }
        }
    };
}

fn main() {}