use crate::hashing::{HashAlgorithm, Keccak256, Keccak384, Sha256};

use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[macro_export]
macro_rules! variable_size_hash {
    ($name:ident, 32) => {
        #[repr(C)]
        #[derive(Debug, FromZeroes, FromBytes, AsBytes, Unaligned)]
        pub struct $name {
            algorithm: HashAlgorithm,
            bytes: [u8; 32],
        }

        impl $name {
            pub fn new(algorithm: HashAlgorithm, bytes: [u8; 32]) -> Self {
                $name { algorithm, bytes }
            }

            pub fn hash_bytes(bytes: &[u8], hash_algorithm: HashAlgorithm) -> Self {
                match hash_algorithm {
                    HashAlgorithm::KECCAK256 => Keccak256::from_bytes(bytes),
                    HashAlgorithm::SHA256 => Sha256::from_bytes(bytes),
                    _ => panic!("Unknown hash algorithm"),
                }
            }
        }
        $crate::serialise!($name);
    };

    ($name:ident, 48) => {
        #[repr(C)]
        #[derive(Debug, FromZeroes, FromBytes, AsBytes, Unaligned)]
        pub struct $name {
            algorithm: HashAlgorithm,
            bytes: [u8; 48],
        }

        impl $name {
            pub fn new(algorithm: HashAlgorithm, bytes: [u8; 48]) -> Self {
                $name { algorithm, bytes }
            }

            pub fn hash_bytes(bytes: &[u8], hash_algorithm: HashAlgorithm) -> Self {
                match hash_algorithm {
                    HashAlgorithm::KECCAK384 => Keccak384::from_bytes(bytes),
                    _ => panic!("Unknown hash algorithm"),
                }
            }
        }

        $crate::serialise!($name);
    };
}

variable_size_hash!(Hash, 32);
variable_size_hash!(Hash384, 48);
