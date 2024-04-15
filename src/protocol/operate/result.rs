
pub struct ActExecRes {
    gas: u32,
    val: Vec<u8>,
    err: Option<Error>,
}



impl ExecResult for ActExecRes {
    fn gasuse(&self) -> u32 { 
        self.gas
    }
    fn retval(&self) -> &[u8] {
        self.val.as_ref()
    }
    fn abort(&self) -> &Option<Error>{
        &self.err
    }
}

impl ActExecRes {
    pub fn new() -> ActExecRes {
        ActExecRes{
            gas: 0,
            val: Vec::new(),
            err: None,
        }
    }
    pub fn wrap(iferr: RetErr) -> Box<ActExecRes> {
        Box::new(ActExecRes{
            gas: 0,
            val: Vec::new(),
            err: iferr.err(),
        })
    }
    pub fn add_gas_use(&mut self, gas: u32) {
        self.gas += gas;
    }
}