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

#[macro_export]
macro_rules! hashable {
    ($t:ty) => {
        $crate::impl_hash_by_type!($t, $crate::hashing::Hash, hash);
        $crate::impl_hash_by_type!($t, $crate::hashing::Hash384, hash384);
        $crate::impl_verify!($t);
    };
}

#[macro_export]
macro_rules! hashable_by_type {
    ($t:ty, $h:ty, $fn_name:ident) => {
        $crate::impl_hash_by_type!($t, $h, $fn_name);
    };
}

#[macro_export]
macro_rules! impl_hash_by_type {
    ($t:ty, $h:ty, $fn_name:ident) => {
        impl $t {
            pub fn $fn_name(&self, hash_algorithm: $crate::hashing::HashAlgorithm) -> $h {
                <$h>::hash_bytes(self.as_bytes(), hash_algorithm)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_verify {
    ($t:ty) => {
        impl $t {
            pub fn verify(&self, _hash: &$crate::hashing::Hash) -> bool {
                unimplemented!()
            }
        }
    };
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::serialise::SerialString;
    use crate::serialise::SerialiseType;

    #[test]
    fn test_hash() {
        let bytes: Vec<u8> = vec![1, 2, 3];
        let hash = Hash::hash_bytes(&bytes, HashAlgorithm::KECCAK256);

        let hash_str: SerialString = hash.into_serial_string(SerialiseType::Base36);
        println!("hash: {}", hash_str.get_string());
        println!("hash debug: {:?}", hash);

        let hash = Hash::hash_bytes(&bytes, HashAlgorithm::SHA256);

        let hash_str: SerialString = hash.into_serial_string(SerialiseType::Base36);
        println!("hash: {}", hash_str.get_string());
        println!("hash debug: {:?}", hash);

        let hash = Hash384::hash_bytes(&bytes, HashAlgorithm::KECCAK384);

        let hash_str: SerialString = hash.into_serial_string(SerialiseType::Base36);
        println!("hash: {}", hash_str.get_string());
        println!("hash debug: {:?}", hash);
    }
}
