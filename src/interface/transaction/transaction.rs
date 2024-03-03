
pub trait TransactionRead {    
    
    fn hash(&self) -> Hash { panic_never_call_this!() }
    fn hash_with_fee(&self) -> Hash { panic_never_call_this!() }

    fn get_type(&self) -> u8;

    fn get_address(&self) -> &Address;
    fn get_fee(&self) -> &Amount { panic_never_call_this!(); }
    fn get_fee_miner_received(&self) -> Amount { panic_never_call_this!() }
    fn get_timestamp(&self) -> &Timestamp { panic_never_call_this!() }

    fn get_reward(&self) -> &Amount { panic_never_call_this!() }
    fn get_message(&self) -> &StringTrim16 { panic_never_call_this!() }
    
    fn get_action_count(&self) -> u16 { panic_never_call_this!() }
    fn get_actions(&self) -> &Vec<Box<dyn Action>> { panic_never_call_this!(); }

    fn get_signs(&self) -> &Vec<Sign> { panic_never_call_this!(); }
    
    fn fee_purity(&self) -> u32 { 0 }
}


pub trait Transaction : TransactionRead {

    fn verify_all_need_signs(&self) -> Option<Error> { panic_never_call_this!() }
    fn verify_target_signs(&self, _: &HashSet<Address>) -> Option<Error> { panic_never_call_this!() }
    fn fill_sign(&mut self,_: &Account) -> Option<Error> { panic_never_call_this!() }
    fn push_action(&mut self, _: Box<dyn Action>) -> Option<Error> { panic_never_call_this!() }

}
