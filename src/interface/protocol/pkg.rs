
pub trait HashPkg {
    fn hash(&self) -> &Hash;
    fn body(&self) -> &[u8];
}


pub trait BlockPkg : HashPkg {
    fn objc(&self) -> &Box<dyn Block>;
}


pub trait TxPkg : HashPkg {
    fn objc(&self) -> &Box<dyn Transaction>;
}

