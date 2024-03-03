


pub trait BlockRead {

    fn hash(&self) -> Hash;

    fn get_timestamp(&self) -> &Timestamp;

    fn get_transaction_count(&self) -> u16;
    fn get_transaction(&self) -> &Vec<Box<dyn Transaction>>;



}



pub trait Block : BlockRead {

}

