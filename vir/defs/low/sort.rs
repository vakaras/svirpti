use super::context::*;

vir_include! { sort =>
    use Sort;
    derive PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize;
}
vir_include! { sort::display =>
    use Sort;
}
