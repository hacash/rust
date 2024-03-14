


pub trait BlockRead {

    fn hash(&self) -> Hash;

    fn timestamp(&self) -> &Timestamp;

    fn transaction_count(&self) -> u16;
    fn transaction(&self) -> &Vec<Box<dyn Transaction>>;



}



pub trait Block : BlockRead {

}

