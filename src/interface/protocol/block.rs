


pub trait BlockRead {

    fn hash(&self) -> Hash;

    fn height(&self) -> &BlockHeight;
    fn timestamp(&self) -> &Timestamp;
    fn prevhash(&self) -> &Hash;
    fn mrklroot(&self) -> &Hash;
    
    

    fn transaction_count(&self) -> u16;
    fn transactions(&self) -> &Vec<Box<dyn Transaction>>;
    fn transaction_hash_list(&self, iswithfee: bool) -> Vec<Hash>;

}



pub trait Block : BlockRead {

    fn push_transaction(&mut self, _: &dyn Transaction) -> Option<Error>;

}

