
pub trait HashBodyPkg {
    fn hash(&self) -> &Hash { panic_never_call_this!() }
    fn body(&self) -> &[u8] { panic_never_call_this!() }
}


pub trait BlockPkg : HashBodyPkg {
    fn objc(&self) -> &Box<dyn Block> { panic_never_call_this!() }
    fn origin(&self) -> BLOCK_ORIGIN { BLOCK_ORIGIN::UNKNOW }
}


pub trait TxPkg : HashBodyPkg {
    fn objc(&self) -> &Box<dyn Transaction> { panic_never_call_this!() }
}

