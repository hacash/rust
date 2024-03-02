
pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
    fn size(&self) -> usize;
}