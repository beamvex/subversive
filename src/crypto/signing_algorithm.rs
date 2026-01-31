#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SigningAlgorithm {
    ED25519,
    RSA,
    ECDSA,
}

impl From<u8> for SigningAlgorithm {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::ED25519,
            1 => Self::RSA,
            2 => Self::ECDSA,
            _ => panic!("Invalid signing algorithm"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_default() {
        let algorithm = SigningAlgorithm::ED25519;
        assert_eq!(algorithm, SigningAlgorithm::ED25519);
    }
}
