

use base58check::*;
use crate::core::account::Account;

// ********* Address *********

pub type Address = Fixed21;
pub const ADDRESS_SIZE: usize = Address::width();

// format
impl Address {

    pub fn form_readable(addr: &str) -> Result<Address, Error> {
        let res = addr.from_base58check();
        if let Err(_) = res {
            return Err("base58check error".to_string())
        }
        let (version, body) = res.unwrap();
        if body.len() != ADDRESS_SIZE - 1 {
            return Err("base58check error".to_string())
        }
        let mut address = Address::default();
        address[0] = version;
        for i in 1..ADDRESS_SIZE {
            address[i] = body[i-1];
        }
        Ok(address)
    }
    
    pub fn readable(&self) -> String {
        let btcon = self.serialize();
        let bts: [u8; ADDRESS_SIZE] = btcon.try_into().unwrap();
        Account::to_readable(&bts)
    }
    

    
}

// ***********************