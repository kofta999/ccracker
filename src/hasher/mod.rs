pub mod md5;
pub mod sha256;

pub trait Hasher {
    fn new() -> Self;
    fn reset(&mut self);
    fn update(&mut self, input: &[u8]);
    fn finalize(&mut self) -> String;
    fn hash(input: &[u8]) -> String;
}
