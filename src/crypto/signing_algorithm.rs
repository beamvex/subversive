#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SigningAlgorithm {
    ED25519,
    RSA,
    ECDSA,
}

impl From<u8> for SigningAlgorithm {
    fn from(value: u8) -> Self {
        match value {
            100 => Self::ED25519,
            101 => Self::RSA,
            102 => Self::ECDSA,
            _ => panic!("Invalid signing algorithm {value}"),
        }
    }
}

impl From<SigningAlgorithm> for u8 {
    fn from(value: SigningAlgorithm) -> Self {
        match value {
            SigningAlgorithm::ED25519 => 100,
            SigningAlgorithm::RSA => 101,
            SigningAlgorithm::ECDSA => 102,
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
