// TODO: Figure out how to move this to a submodule.
#[macro_export]
macro_rules! derive_string_symbol {
    ($name: ident) => {
        #[derive(
            Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
        )]
        pub struct $name(String);

        impl $name {
            pub fn as_string(&self) -> String {
                self.0.clone()
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

pub mod common;

include!(concat!(env!("OUT_DIR"), "/vir.rs"));
