use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, AsBytes, Unaligned, FromZeroes, FromBytes)]
pub struct AlgorithmType(u8);

impl AlgorithmType {
    pub const ED25519: AlgorithmType = AlgorithmType(0);
    pub const RSA: AlgorithmType = AlgorithmType(1);
    pub const ECDSA: AlgorithmType = AlgorithmType(2);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_default() {
        let algorithm = AlgorithmType::default();
        assert_eq!(algorithm, AlgorithmType::ED25519);
    }
}
