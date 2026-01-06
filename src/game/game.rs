
use ed25519_dalek::Signature;
use crate::utils::bytes_to_base36;

pub fn play(signature1: &[u8], signature2: &[u8]) -> i32 {
    let signature1: &[u8; 64] = signature1
        .try_into()
        .expect("ed25519 signature must be 64 bytes");
    let signature2: &[u8; 64] = signature2
        .try_into()
        .expect("ed25519 signature must be 64 bytes");
    
    let signature1 = bytes_to_base36(signature1);
    let signature2 = bytes_to_base36(signature2);
    println!("signature1: {}", signature1);
    println!("signature2: {}", signature2);
    0
}

mod tests {
    use crate::game::play;
    use crate::crypto::generate_key;
    use crate::crypto::sign;

    #[test]
    fn test_play() {
        let (private_key1, public_key1) = generate_key();

        let (private_key2, public_key2) = generate_key();

        let data = b"test";
        let signature1 = sign(data, &private_key1);
        let data = b"test";
        let signature2 = sign(data, &private_key2);
        assert_eq!(play(&signature1, &signature2), 0);
    }
}