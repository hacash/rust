
pub struct ActExecRes {
    gas: i64,
    val: Vec<u8>,
    err: Option<Error>,
}



impl ExecResult for ActExecRes {
    fn gasuse(&self) -> i64 { 
        self.gas
    }
    fn retval(&self) -> &[u8] {
        self.val.as_ref()
    }
    fn asval(self) -> Vec<u8> {
        self.val
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
    pub fn create(gas: i64, dt: Ret<Vec<u8>>) -> Box<ActExecRes> {
        let (v, e) = match dt {
            Ok(v) => (v, None),
            Err(e) => (vec![], Some(e)),
        };
        Box::new(ActExecRes{
            gas: gas,
            val: v,
            err: e,
        })
    }
    pub fn wrap(iferr: RetErr) -> Box<ActExecRes> {
        Box::new(ActExecRes{
            gas: 0,
            val: Vec::new(),
            err: iferr.err(),
        })
    }
    pub fn add_gas_use(&mut self, gas: u32) {
        self.gas += gas as i64;
    }
}