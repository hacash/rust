


pub trait BlockRead : Serialize {

    fn hash(&self) -> Hash { panic_never_call_this!() }

    fn height(&self) -> &BlockHeight { panic_never_call_this!() }
    fn timestamp(&self) -> &Timestamp { panic_never_call_this!() }
    fn prevhash(&self) -> &Hash { panic_never_call_this!() }
    fn mrklroot(&self) -> &Hash { panic_never_call_this!() }
    
    fn transaction_count(&self) -> u16 { panic_never_call_this!() }
    fn transactions(&self) -> Vec<&dyn Transaction> { panic_never_call_this!() }
    fn transaction_hash_list(&self, iswithfee: bool) -> Vec<Hash> { panic_never_call_this!() }

}



pub trait Block : BlockRead + Parse {

    fn push_transaction(&mut self, _: &dyn Transaction) -> Option<Error> { panic_never_call_this!() }

}

