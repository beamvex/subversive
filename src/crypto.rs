#[path = "crypto/generate.rs"]
pub mod generate;
/*
#[path = "crypto/sign.rs"]
mod sign;

#[path = "crypto/verify.rs"]
mod verify;

pub use sign::sign;
pub use verify::verify;
*/

pub use generate::generate_key;
