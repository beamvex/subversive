
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
    /// compare signatures return 1 if signature1 > signature2, -1 if signature1 < signature2, 0 if equal
    if signature1 > signature2 {
        1
    } else if signature1 < signature2 {
        -1
    } else {
        0
    }
}

mod tests {
    use crate::game::play;
    use crate::crypto::generate_key;
    use crate::crypto::sign;
    use crate::utils::{base36_to_bytes_32, bytes_to_base36};

    #[test]
    fn test_play() {

        let private_key1 = base36_to_bytes_32("3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833");
        let private_key2 = base36_to_bytes_32("26j1x2pu9o8kae53tl7jnhu5osmcu8au4glzi5jno5027zr08m");

        println!("private_key1: {}", bytes_to_base36(&private_key1));
        println!("private_key2: {}", bytes_to_base36(&private_key2));

        let data = b"test";
        let signature1 = sign(data, &private_key1);
        let data = b"test";
        let signature2 = sign(data, &private_key2);
        assert_eq!(play(&signature1, &signature2), -1);
    }
}