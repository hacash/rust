
pub struct ExecEnvObj {
    pdhei: u64,
    pdhash: Hash,
    mainaddr: Address,
    txfee: Amount,
}


impl ExecEnvObj {
    pub fn new(pdhei: u64, tx: &dyn TransactionRead) -> ExecEnvObj {
        ExecEnvObj{
            pdhei: pdhei,
            pdhash: Hash::new(),
            mainaddr: tx.address().clone(),
            txfee: tx.fee().clone(),
        }
    }
}


impl ExecEnv for ExecEnvObj {

    fn pending_height(&self) -> u64 {
        self.pdhei
    }
    fn pending_hash(&self) -> &Hash {
        &self.pdhash
    }
    fn tx_fee(&self) -> &Amount {
        &self.txfee
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
