
pub trait TransactionRead : Field + Send + Sync + dyn_clone::DynClone {    

    fn hash(&self) -> Hash { panic_never_call_this!() }
    fn hash_with_fee(&self) -> Hash { panic_never_call_this!() }

    fn ty(&self) -> u8 { panic_never_call_this!() }

    fn address(&self) -> Ret<Address> { panic_never_call_this!() }
    fn addrlist(&self) -> &AddrOrList{ panic_never_call_this!() }
    fn fee(&self) -> &Amount { panic_never_call_this!(); }
    fn gas_max(&self) -> u8 { panic_never_call_this!(); }
    fn timestamp(&self) -> &Timestamp { panic_never_call_this!() }

    fn reward(&self) -> &Amount { panic_never_call_this!() }
    fn message(&self) -> &StringTrim16 { panic_never_call_this!() }
    
    fn action_count(&self) -> u16 { panic_never_call_this!() }
    fn actions(&self) -> &Vec<Box<dyn Action>> { panic_never_call_this!(); }

    fn signs(&self) -> &Vec<Sign> { panic_never_call_this!(); }
    
    // fn fee_purity(&self) -> u32 { 0 }
    fn req_sign(&self) -> Ret<HashSet<Address>> { panic_never_call_this!(); }
    fn fee_got(&self) -> Amount { panic_never_call_this!(); } // fee_miner_received
    fn burn_90(&self) -> bool { panic_never_call_this!(); } // burn_90_percent_fee
}


pub trait Transaction : TransactionRead + TxExec + Send + Sync {

    fn as_read(&self) -> &dyn TransactionRead;

    // fn verify_all_need_signs(&self) -> Option<Error> { panic_never_call_this!() }
    // fn verify_target_signs(&self, _: &HashSet<Address>) -> Option<Error> { panic_never_call_this!() }
    fn fill_sign(&mut self,_: &Account) -> RetErr { panic_never_call_this!() }
    fn push_action(&mut self, _: Box<dyn Action>) -> RetErr { panic_never_call_this!() }

}

dyn_clone::clone_trait_object!(TransactionRead);
dyn_clone::clone_trait_object!(Transaction);
