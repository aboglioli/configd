use sha2::{Digest, Sha256};

use crate::domain::Hasher;

pub struct Sha256Hasher;

impl Sha256Hasher {
    pub fn new() -> Sha256Hasher {
        Sha256Hasher
    }
}

impl Hasher for Sha256Hasher {
    fn hash(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();

        hex::encode(result)
    }
}
