use crate::serialise::SerialiseType;

pub trait Serialiser {
    fn get_serialise_type(&self) -> SerialiseType;
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
    };
}
