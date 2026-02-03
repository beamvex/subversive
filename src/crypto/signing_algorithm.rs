#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SigningAlgorithm {
    ED25519,
    RSA,
    ECDSA,
}

impl TryFrom<u8> for SigningAlgorithm {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            100 => Ok(Self::ED25519),
            101 => Ok(Self::RSA),
            102 => Ok(Self::ECDSA),
            _ => Err(format!("Invalid signing algorithm {value}")),
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
