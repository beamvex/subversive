use crate::serialise::{Base36, SerialiseType};

pub trait Serialiser {
    fn get_serialise_type(&self) -> SerialiseType;

    fn into_base36(&self) -> Base36 {
        panic!("not implemented");
    }
}

impl std::fmt::Debug for dyn Serialiser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Serialiser")
    }
}

#[macro_export]
macro_rules! serialise {
    ($t:ty) => {
        impl $t {
            pub fn serialise(
                &self,
                serialise_type: $crate::serialise::SerialiseType,
            ) -> impl $crate::serialise::Serialiser {
                match serialise_type {
                    $crate::serialise::SerialiseType::Base36 => {
                        let b36: $crate::serialise::Base36 = (self).into();
                        b36
                    }
                    _ => panic!("unknown serialise type"),
                }
            }
        }

        impl $t {
            pub fn from(value: &dyn $crate::serialise::Serialiser) -> $t {
                match value.get_serialise_type() {
                    $crate::serialise::SerialiseType::Base36 => {
                        let b36: $crate::serialise::Base36 = value.into_base36();
                        let result: $t = (&b36).into();
                        result
                    }
                    _ => panic!("unknown serialise type"),
                }
            }
        }
    };
}
