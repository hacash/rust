

pub trait BlockPkg {
    fn hash(&self) -> &Hash;
    fn body(&self) -> &[u8];
    fn inst(&self) -> &Box<dyn Block>;
}

