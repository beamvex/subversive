use crate::serialise::SerialiseType;

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
        $crate::impl_from!($t);
        $crate::impl_into!($t);
    };
}

#[macro_export]
macro_rules! impl_from {
    ($t:ty) => {
        impl From<&$crate::serialise::SerialString> for $t {
            fn from(value: &$crate::serialise::SerialString) -> Self {
                match value.get_serialise_type() {
                    $crate::serialise::SerialiseType::Base36 => {
                        let bytes =
                            $crate::serialise::base36::Base36::from_base36(&value.get_string(), 0);
                        <$t>::from_bytes(&bytes)
                    }
                    _ => {
                        panic!("Unsupported serialise type");
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_into {
    ($t:ty) => {
        impl $t {
            pub fn into_serial_string(
                &self,
                serialise_type: $crate::serialise::SerialiseType,
            ) -> $crate::serialise::SerialString {
                match serialise_type {
                    $crate::serialise::SerialiseType::Base36 => {
                        let bytes = self.as_bytes();
                        let string = $crate::serialise::base36::Base36::to_base36(&bytes);
                        $crate::serialise::SerialString::new(serialise_type, string)
                    }
                    _ => {
                        panic!("Unsupported serialise type");
                    }
                }
            }
        }
    };
}
