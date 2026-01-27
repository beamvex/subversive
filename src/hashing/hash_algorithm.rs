#[derive(Debug, Copy, Clone, Default)]
pub enum HashAlgorithm {
    #[default]
    Keccak256,
    Sha256,
}
