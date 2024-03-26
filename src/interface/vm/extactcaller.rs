
pub trait ExtActCaller {

    fn cutout(&self, _: &[u8]) -> Result<Vec<u8>, Error>;

    fn execute(&mut self, _: &[u8]) -> Result<Box<dyn ExecResult>, Error>;

}



