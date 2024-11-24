pub mod md5;

pub trait Hasher {
    fn new() -> Self;
    fn reset(&mut self);
    fn update(&mut self, input: &[u8]);
    fn finalize(&mut self) -> String;
}
