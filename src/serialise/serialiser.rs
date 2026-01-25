use crate::serialise::SerialiseType;

pub trait Serialiser {
    fn get_serialise_type(&self) -> SerialiseType;
}

#[macro_export]
macro_rules! serialise {
    ($t:ty) => {
        impl $t {
            pub fn serialise(
                serialise_type: $crate::serialise::SerialiseType,
                value: &$t,
            ) -> impl $crate::serialise::Serialiser {
                match serialise_type {
                    $crate::serialise::SerialiseType::Base36 => {
                        let b36: $crate::serialise::Base36 = (&value).into();
                        b36
                    }
                    _ => panic!("unknown serialise type"),
                }
            }
        }
    };
}
