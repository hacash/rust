


pub trait BlockRead : Serialize + Send + Sync + dyn_clone::DynClone {

    fn hash(&self) -> Hash { panic_never_call_this!() }

    fn version(&self) -> &Uint1 { panic_never_call_this!() }
    fn height(&self) -> &BlockHeight { panic_never_call_this!() }
    fn timestamp(&self) -> &Timestamp { panic_never_call_this!() }
    fn nonce(&self) -> &Uint4 { panic_never_call_this!() }
    fn difficulty(&self) -> &Uint4 { panic_never_call_this!() } 
    fn prevhash(&self) -> &Hash { panic_never_call_this!() }
    fn mrklroot(&self) -> &Hash { panic_never_call_this!() }
    
    fn transaction_count(&self) -> &Uint4 { panic_never_call_this!() }
    fn transactions(&self) -> &Vec<Box<dyn Transaction>> { panic_never_call_this!() }
    fn transaction_hash_list(&self, hash_with_fee: bool) -> Vec<Hash> { panic_never_call_this!() }

}

pub trait Block : BlockRead + Parse + Send + Sync + dyn_clone::DynClone {

    fn as_read(&self) -> &dyn BlockRead { panic_never_call_this!() }

    fn update_mrklroot(&mut self) { panic_never_call_this!() }
    fn set_nonce(&mut self, _: Uint4) { panic_never_call_this!() }
    fn set_mrklroot(&mut self, _: Hash) { panic_never_call_this!() }
    
    fn replace_transaction(&mut self, _: usize, _: Box<dyn Transaction>) -> RetErr { panic_never_call_this!() }
    fn push_transaction(&mut self, _: Box<dyn Transaction>) -> RetErr { panic_never_call_this!() }

}


dyn_clone::clone_trait_object!(BlockRead);
dyn_clone::clone_trait_object!(Block);