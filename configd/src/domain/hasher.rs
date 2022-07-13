pub trait Hasher {
    fn hash(&self, data: &[u8]) -> String;
}
