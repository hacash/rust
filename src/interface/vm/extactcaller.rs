
pub trait ExtActCaller {
    // fn cutout(&self, _: &[u8]) -> Result<Vec<u8>, Error>;
    fn call(&mut self, kind_and_body: Vec<u8>, depth: i8) -> Ret<(i64, Vec<u8>)>;
}


pub trait OutStoragerRead {
    // forever store
    fn get(&self, key: &[u8]) -> Ret<Option<Vec<u8>>>;
}


pub trait OutStorager: OutStoragerRead {
    // forever store
    fn set(&mut self, key: &[u8], value: Vec<u8>) -> RetErr;
    fn del(&mut self, key: &[u8]) -> RetErr;
}


pub trait OutContext: ExtActCaller + OutStorager {

}









