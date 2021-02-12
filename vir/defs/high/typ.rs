use super::context::*;

vir_include! { typ =>
    use Type;
    use DomainType;
    derive PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize;
}
vir_include! { typ::display =>
    use Type;
    use DomainType;
}
