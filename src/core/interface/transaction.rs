

use super::super::Error;
use super::field::*;
use super::super::field::Address;




pub trait Transaction : Field {
    fn get_type(&self) -> u8;
    fn fee_purity(&self) -> u64 { 0 }
    
}



