
pub trait ActExecRet {
    fn gasuse(&self) -> i32;
    fn value(&self) -> Vec<u8> { vec![] }
    fn abort(&self) -> Option<Error> { None }
}



pub trait ExtActCaller {

    fn cutout(&self, _: &[u8]) -> Result<Vec<u8>, Error>;

    fn execute(&mut self, _: &[u8]) -> Result<Box<dyn ActExecRet>, Error>;

}