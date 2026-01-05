#[path = "crypto/generate.rs"]
mod generate;

#[path = "crypto/sign.rs"]
mod sign;

pub use generate::generate_key;
pub use sign::sign;
