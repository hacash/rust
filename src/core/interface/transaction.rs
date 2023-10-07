use std::collections::HashSet;

use super::super::Error;
use super::field::*;
use super::action::*;
use super::super::component::*;
use super::super::field::*;
use super::super::field::Address;




pub trait TransactionRead : Field {
    
    fn get_type(&self) -> u8;

    fn fee_purity(&self) -> u64 { 0 }
    
    fn get_timestamp(&self) -> &Timestamp { panic_never_call_this!() }
	fn get_address(&self) -> &Address { panic_never_call_this!() }
    fn get_reward(&self) -> &Amount { panic_never_call_this!() }
	fn get_signs(&self) -> &Vec<Sign> { panic_never_call_this!() }
    fn get_fee(&self) -> &Amount { panic_never_call_this!() }
    fn get_fee_of_miner_real_received(&self) -> Amount { panic_never_call_this!() }
    fn get_message(&self) -> &StringTrim16 { panic_never_call_this!() }
    fn get_action_count(&self) ->&Uint2 { panic_never_call_this!() }
	fn get_actions(&self) -> &Vec<Box<dyn Action>> { panic_never_call_this!() }

}


pub trait Transaction : TransactionRead {

    fn verify_all_signs(&self) -> Option<Error> { panic_never_call_this!() }
    fn verify_target_signs(&self, _: &HashSet<Address>) -> Option<Error> { panic_never_call_this!() }

    fn append_action(&mut self, _: Box<dyn Action>) -> Option<Error> { panic_never_call_this!() }


}





