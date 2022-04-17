use base64;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha2::{Digest, Sha256};

fn main() {
    let mut csp_rng = ChaCha20Rng::from_entropy();
    let mut data = [0u8; 32];
    csp_rng.fill_bytes(&mut data);
    println!("raw_binary = {:?}", data);

    let mut hasher256 = Sha256::new();
    hasher256.update(data);
    let hashed: String = format!("{:x}", hasher256.finalize());
    println!("hashed = {}", hashed);

    let base64_encoded = base64::encode(data);
    println!("base64_encoded = {}", base64_encoded);

    let base64_decoded = base64::decode(base64_encoded).unwrap();
    println!("base64_decoded = {:?}", base64_decoded);
}
