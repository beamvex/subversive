use crate::hashing::HashAlgorithm;

use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[macro_export]
macro_rules! variable_size_hash {
    ($name:ident, $len:expr) => {
        #[repr(C)]
        #[derive(Debug, FromZeroes, FromBytes, AsBytes, Unaligned)]
        pub struct $name {
            algorithm: HashAlgorithm,
            bytes: [u8; $len],
        }

        impl $name {
            pub fn new(algorithm: HashAlgorithm, bytes: [u8; $len]) -> Self {
                $name { algorithm, bytes }
            }
        }
    };
}

variable_size_hash!(Hash256, 32);
variable_size_hash!(Hash384, 48);
