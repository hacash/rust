
// NumUInt

pub type NumUInt1 = BytesFixed1;
pub type NumUInt2 = BytesFixed2;
pub type NumUInt3 = BytesFixed3;
pub type NumUInt4 = BytesFixed4;
pub type NumUInt5 = BytesFixed5;
pub type NumUInt8 = BytesFixed8;

// Bool ***********************

pub type Bool = BytesFixed1;

impl Bool {

    pub fn from(v: bool) -> Bool {
        let mut var = Bool::new();
        var.set(v);
        var
    }
    pub fn set(&mut self, v: bool) {
        self.bytes[0] = match v { true => 1u8, false => 0u8 };
    }
    pub fn check(&self) -> bool {
        match self.to_u8() {
            0u8 => false,
            _ => true,
        }
    }
}

// ChannelId ***********************

pub type ChannelId = BytesFixed16;
pub const CHANNEL_ID_SIZE: usize = ChannelId::length();

// Diamond ***********************

pub const DIAMOND_NAME_VALID_CHARS: &[u8; 16]  = b"WTYUIAHXVMEKBSZN";
pub type DiamondName = BytesFixed6;
pub type DiamondNumber = NumUInt3;
pub type DiamondVisualGene = BytesFixed10;

impl DiamondName {
    pub fn name(&self) -> String {
        self.to_string()
    }
    pub fn is_diamond_name(stuff: impl AsRef<[u8]>) -> bool {
        let v = stuff.as_ref();
        if v.len() != 6 {
            return false
        }
        // check in array
        for i in v {
            let mut ok = false;
            for a in DIAMOND_NAME_VALID_CHARS {
                if i == a {
                    ok = true;
                    break
                }
            }
            if !ok {
                return false
            }
        }
        true
    }
}

// Address ***********************

use base58check::*;

pub type Address = BytesFixed21;
pub const ADDRESS_SIZE: usize = Address::length();
// format
impl Address {

    pub fn form_readable(addr: &String) -> Result<Address, Error> {
        let res = addr.from_base58check();
        if let Err(_) = res {
            return Err("base58check error".to_string())
        }
        let (version, body) = res.unwrap();
        if body.len() != ADDRESS_SIZE - 1 {
            return Err("base58check error".to_string())
        }
        let mut address = Address::new();
        address[0] = version;
        for i in 1..ADDRESS_SIZE {
            address[i] = body[i-1];
        }
        Ok(address)
    }
    
    pub fn to_readable(&self) -> String {
        let version = self.bytes[0];
        self.bytes[1..].to_base58check(version)
    }
    
}

// ***********************