
// NumUInt

pub type NumUInt1 = BytesFixed1;
pub type NumUInt2 = BytesFixed2;
pub type NumUInt3 = BytesFixed3;
pub type NumUInt4 = BytesFixed4;
pub type NumUInt5 = BytesFixed5;
pub type NumUInt8 = BytesFixed8;

// NumFloat

pub type UnsafeNumFloat4 = BytesFixed4;
pub type UnsafeNumFloat8 = BytesFixed8;

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

// Lockbls

pub type LockblsId = BytesFixed18;
pub const LOCKBLS_ID_SIZE: usize = LockblsId::length();

// Satoshi

pub type Satoshi = NumUInt8;
impl Satoshi {}

// lending

pub type DiamondSyslendId = BytesFixed14;
pub type BitcoinSyslendId = BytesFixed15;
pub type UserLendingId = BytesFixed17;

pub const DIAMOND_SYSLEND_ID_SIZE: usize = DiamondSyslendId::length();
pub const BITCOIN_SYSLEND_ID_SIZE: usize = BitcoinSyslendId::length();
pub const USER_LENDING_ID_SIZE: usize = UserLendingId::length();

// BlockHeight Timestamp ***********

pub type BlockHeight = NumUInt5;
pub type Timestamp = NumUInt5;
pub const BLOCK_HEIGHT_SIZE: usize = BlockHeight::length();
pub const TIMESTAMP_SIZE: usize = Timestamp::length();
impl BlockHeight {}
impl Timestamp {}

// Hash ***********************

pub type Hash = BytesFixed32;
pub type HashHalf = BytesFixed16;
pub type HashNonce = BytesFixed8;
const HASH_SIZE: usize = Hash::length();
const HASH_HALF_SIZE: usize = HashHalf::length();
const HASH_NONCE_SIZE: usize = HashNonce::length();
impl Hash {

    pub fn from(v: impl AsRef<[u8]>) -> Hash {
        let v = v.as_ref();
        if v.len() != HASH_SIZE {
            panic!("Hash.from() size error.")
        }
        let v: [u8; HASH_SIZE] = v.try_into().unwrap();
        Hash{
            bytes: v,
        }
    }

    pub fn get_half(&self) -> HashHalf {
        let pt: [u8; HASH_HALF_SIZE] = self.bytes[0..HASH_HALF_SIZE].try_into().unwrap();
        <BytesFixed16 as Field>::from(pt)
    }

    pub fn get_nonce(&self) -> HashNonce {
        let pt: [u8; HASH_NONCE_SIZE] = self.bytes[0..HASH_NONCE_SIZE].try_into().unwrap();
        <BytesFixed8 as Field>::from(pt)
    }

}

// Diamond ***********************

pub const DIAMOND_NAME_VALID_CHARS: &[u8; 16]  = b"WTYUIAHXVMEKBSZN";
pub type DiamondName = BytesFixed6;
pub type DiamondNumber = NumUInt3;
pub type DiamondVisualGene = BytesFixed10;

impl DiamondName {
    pub fn name(&self) -> String {
        self.to_string()
    }
    pub fn is_valid(stuff: impl AsRef<[u8]>) -> bool {
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