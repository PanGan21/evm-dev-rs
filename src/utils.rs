use sha3::{Digest, Keccak256};

pub fn sha3_hash(data: &[u8]) -> [u8; 32] {
    if data.is_empty() {
        let result = [0; 32];
        result
    } else {
        // create hash
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let result = hasher.finalize();

        result.into()
    }
}
