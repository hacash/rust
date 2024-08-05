

use base58check::*;
use crate::core::account::Account;

// ********* Address *********

pub type Address = Fixed21;
pub const ADDRESS_SIZE: usize = Address::width();
pub const ADDRVER_PRIVAKEY: u8 = 1;
pub const ADDRVER_MULTISIG: u8 = 2;
pub const ADDRVER_CONTRACT: u8 = 3;

// format
impl Address {
    
    pub fn min() -> Address {
        Fixed21 {
            bytes: [0u8; 21],
        }
    }

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
    
    pub fn version(&self) -> u8 {
        self[0] + 1
    }


    
}




// ***********************


/**
 * Address List
 */
 StructFieldList!{ AddressListW1, 
	count, Uint1, lists, Address
 }



 StructFieldRevMarkEnum!{ AddrOrPtr,
    Addr, Address,
    Ptr, Uint1,
    10 // if Ptr: (buf[0] < 10)
}


impl Copy for AddrOrPtr { }

impl AddrOrPtr {

    pub fn by_addr(adr: Address) -> AddrOrPtr {
        Self::Addr(adr)
    }

    pub fn real(&self, list: &AddrOrList) -> Ret<Address> {
        get_real_addr(list, self)
    }

}


StructFieldRevMarkEnum!{ AddrOrList,
    Addr, Address,
    List, AddressListW1,
    10 // if Ptr: (buf[0] < 10)
}

impl AddrOrList {

    pub fn by_addr(adr: Address) -> AddrOrList {
        AddrOrList::Addr(adr)
    }

    pub fn main(&self) -> Ret<Address> {
        let adr = match self {
            Self::Addr(adr) => adr.clone(),
            Self::List(ary) => match ary.count().value() {
                0 => return Err("Address list is empty".to_owned()),
                _ => ary[0].clone(),
            }
        };
        Ok(adr)
    }

    pub fn real(&self, ap: &AddrOrPtr) -> Ret<Address> {
        get_real_addr(self, ap)
    }

}




fn get_real_addr(list: &AddrOrList, ptr: &AddrOrPtr) -> Ret<Address> {
    let idx = match ptr {
        AddrOrPtr::Addr(adr) => return Ok(adr.clone()),
        AddrOrPtr::Ptr(ptr) => ptr.value() as usize,
    };

    let mut aryo: Vec<Address> = vec![];
    let ary = match list {
        AddrOrList::Addr(adr) => {
            aryo = vec![adr.clone()];
            &aryo
        },
        AddrOrList::List(ary) => ary.list(),
    };
    // index
    match idx >= ary.len() {
        true => return Err("Address list overflow".to_owned()),
        _ => Ok(ary[idx].clone()),
    }
}

