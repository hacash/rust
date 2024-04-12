
pub struct ExecEnvObj {
    pending_height: u64,
}


impl ExecEnvObj {
    pub fn new(pdhei: u64, tx: &dyn TransactionRead) -> ExecEnvObj {
        ExecEnvObj{
            pending_height: pdhei,
        }
    }
}


impl ExecEnv for ExecEnvObj {

}
