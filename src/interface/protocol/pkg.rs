
pub trait HashBodyPkg {
    fn hash(&self) -> &Hash { panic_never_call_this!() }
    fn body(&self) -> &BytesW4 { panic_never_call_this!() }
}


pub trait BlockPkg : HashBodyPkg + Send + Sync + dyn_clone::DynClone {
    fn objc(&self) -> &Box<dyn Block>;
    fn origin(&self) -> BLOCK_ORIGIN { BLOCK_ORIGIN::UNKNOW }
}


pub trait TxPkg : HashBodyPkg + Send + Sync + dyn_clone::DynClone {
    fn objc(&self) -> &Box<dyn Transaction> { panic_never_call_this!() }
    fn burn_90(&self) -> bool { false }
    fn fee_purity(&self) -> u32 { 0 }
    fn tx_pool_group(&self) -> u8 { 0 } // 0:normal    1:diamond create    2:
} 


dyn_clone::clone_trait_object!(TxPkg);
dyn_clone::clone_trait_object!(BlockPkg);


