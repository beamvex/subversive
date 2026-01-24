#[repr(u32)]
#[derive(Debug, Default, Immutable, IntoBytes)]
pub enum Algorithm {
    #[default]
    Ed25519 = 0,
}
