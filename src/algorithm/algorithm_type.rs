use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct AlgorithmType(u8);

impl AlgorithmType {
    pub const ED25519: Self = Self(0);
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
