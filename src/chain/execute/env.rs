
pub struct ExecEnvObj<'a> {
    fastsync: bool,
    pdhei: u64,
    pdhash: Hash,
    mainaddr: Address,
    tx: &'a dyn TransactionRead,
}


impl ExecEnvObj<'_> {
    pub fn new<'a>(pdhei: u64, tx: &'a dyn TransactionRead) -> ExecEnvObj {
        ExecEnvObj{
            fastsync: false,
            pdhei: pdhei,
            pdhash: Hash::default(),
            mainaddr: tx.address().clone(),
            tx: tx,
        }
    }
}


impl ExecEnv for ExecEnvObj<'_> {

    fn pending_height(&self) -> u64 {
        self.pdhei
    }
    fn pending_hash(&self) -> &Hash {
        &self.pdhash
    }
    fn tx_fee(&self) -> &Amount {
        self.tx.fee()
    }
    fn main_address(&self) -> &Address {
        &self.mainaddr
    }
    fn check_signature(&self, adr: &Address) -> RetErr {
        transaction::verify_target_signature(adr, self.tx)
    }
    fn call_depth(&self) -> u32 {
        0
    }
    fn fast_sync(&self) -> bool {
        self.fastsync
    }
}
