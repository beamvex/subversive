#[repr(u8)]
#[derive(Debug, Default, Eq, PartialEq)]
pub enum AlgorithmType {
    #[default]
    Ed25519 = 0,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_default() {
        let algorithm = AlgorithmType::default();
        assert_eq!(algorithm, AlgorithmType::Ed25519);
    }
}
