use std::collections::HashSet;

use super::super::Error;
use super::field::*;
use super::super::field::Address;




pub trait Action : Field {
    fn get_kind(&self) -> u16;
    fn is_burning_90_persent_tx_fee(&self) -> bool { false }
    fn request_sign_addresses(&self) -> HashSet<Address> { HashSet::new() }
}


