#[cfg(test)]
mod tests {
    use base_xx::ByteVec;
    use simple_sign::{Ed25519Signer, Signature, Signer};
    use slahasher::Hash;

    #[test]
    fn test_throughput() {
        let signer = Ed25519Signer::new_random();
        let mut last_signature = Signature::default();
        let mut wincount = 0;
        for i in 0..100_000 {
            if i % 10_000 == 0 {
                println!("Throughput test: {i}");
            }
            let message = format!("test message {i}");
            let hash = Hash::try_hash(
                &ByteVec::new(message.as_bytes().to_vec()),
                slahasher::HashAlgorithm::KECCAK512,
            )
            .unwrap_or_else(|e| unreachable!("broke {e}"));
            let signature = signer
                .sign(&hash)
                .unwrap_or_else(|e| unreachable!("broke {e}"));

            if last_signature > signature || last_signature == Signature::default() {
                wincount += 1;
                last_signature = signature;
            }
        }
        println!("Win count: {wincount}");
    }
}
