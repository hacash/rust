
pub struct ExecEnvObj {
    pdhei: u64,
    mainaddr: Address,
}


impl ExecEnvObj {
    pub fn new(pdhei: u64, tx: &dyn TransactionRead) -> ExecEnvObj {
        ExecEnvObj{
            pdhei: pdhei,
            mainaddr: tx.address().clone(),
        }
    }
}


impl ExecEnv for ExecEnvObj {

    fn pending_height(&self) -> u64 {
        self.pdhei
    }
    fn main_address(&self) -> &Address {
        &self.mainaddr
    }
    fn check_signature(&self, _: &Address) -> bool {
        true
    }
    fn call_depth(&self) -> u32 {
        0
    }
}
