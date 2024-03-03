
pub trait BlockRead {
    
    fn hash(&self) -> Hash;

    fn get_timestamp(&self) -> &Timestamp;
    
    
}    
    


pub trait Block : BlockRead {

}
