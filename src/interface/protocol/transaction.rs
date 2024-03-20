
pub trait TransactionRead : Field {    
    
    fn hash(&self) -> Hash { panic_never_call_this!() }
    fn hash_with_fee(&self) -> Hash { panic_never_call_this!() }

    fn ty(&self) -> u8;

    fn address(&self) -> &Address;
    fn fee(&self) -> &Amount { panic_never_call_this!(); }
    fn fee_miner_received(&self) -> Amount { panic_never_call_this!() }
    fn timestamp(&self) -> &Timestamp { panic_never_call_this!() }

    fn reward(&self) -> &Amount { panic_never_call_this!() }
    fn message(&self) -> &StringTrim16 { panic_never_call_this!() }
    
    fn action_count(&self) -> u16 { panic_never_call_this!() }
    fn actions(&self) -> &Vec<Box<dyn VMAction>> { panic_never_call_this!(); }

    fn signs(&self) -> &Vec<Sign> { panic_never_call_this!(); }
    
    fn fee_purity(&self) -> u32 { 0 }
}


pub trait Transaction : TransactionRead {

    fn verify_all_need_signs(&self) -> Option<Error> { panic_never_call_this!() }
    fn verify_target_signs(&self, _: &HashSet<Address>) -> Option<Error> { panic_never_call_this!() }
    fn fill_sign(&mut self,_: &Account) -> Option<Error> { panic_never_call_this!() }
    fn push_action(&mut self, _: Box<dyn Action>) -> Option<Error> { panic_never_call_this!() }

}
