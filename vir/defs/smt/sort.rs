use super::context::*;

vir_include! { sort =>
    use Sort;
    use WithSort;
    derive PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize;
}
vir_include! { sort::rsmt =>
    use Sort;
}
vir_include! { sort::display =>
    use Sort;
}
