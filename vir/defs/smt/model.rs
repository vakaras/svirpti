use super::context::*;

vir_include! { model =>
    use Value;
    use ModelItemArg;
    use ModelItem;
    use Model;
    derive PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize;
}
