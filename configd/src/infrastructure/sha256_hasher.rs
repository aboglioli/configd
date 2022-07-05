use sha2::{Digest, Sha256};

use crate::domain::Hasher;

pub struct Sha256Hasher;

impl Sha256Hasher {
    pub fn new() -> Sha256Hasher {
        Sha256Hasher
    }
}

impl Hasher for Sha256Hasher {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();

        result.to_vec()
    }
}
