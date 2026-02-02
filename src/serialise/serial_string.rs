use crate::serialise::{base36::Base36, SerialiseType};

#[derive(Debug)]
pub struct SerialString {
    serialise_type: SerialiseType,
    string: String,
}

impl SerialString {
    pub fn new(serialise_type: SerialiseType, string: String) -> Self {
        Self {
            serialise_type,
            string,
        }
    }

    pub fn get_serialise_type(&self) -> SerialiseType {
        self.serialise_type
    }

    pub fn get_string(&self) -> &String {
        &self.string
    }
}

impl std::fmt::Display for SerialString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

#[macro_export]
macro_rules! serialisable {
    ($t:ty) => {
        // base36
        $crate::impl_to_base36!($t);
        $crate::impl_from_base36!($t);

        // generic
        //$crate::impl_from!($t);
    };
}

#[macro_export]
macro_rules! impl_from {
    ($t:ty) => {
        impl From<&$crate::serialise::SerialString> for $t {
            fn from(value: &$crate::serialise::SerialString) -> Self {
                match value.get_serialise_type() {
                    $crate::serialise::SerialiseType::Base36 => {
                        let base36: &$crate::serialise::base36::Base36 = value.into();

                        base36.into()
                    }
                    _ => {
                        panic!("Unsupported serialise type");
                    }
                }
            }
        }
    };
}
