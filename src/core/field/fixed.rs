// String

pub type StringTrim16 = Fixed16;

// Uint

pub type Uint1 = Fixed1;
pub type Uint2 = Fixed2;
pub type Uint3 = Fixed3;
pub type Uint4 = Fixed4;
pub type Uint5 = Fixed5;
pub type Uint8 = Fixed8;

// NumFloat

pub type UnsafeNumFloat4 = Fixed4;
pub type UnsafeNumFloat8 = Fixed8;

// Bool ***********************

pub type Bool = Fixed1;
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

pub type ChannelId = Fixed16;
pub const CHANNEL_ID_SIZE: usize = ChannelId::length();

// Lockbls

pub type LockblsId = Fixed18;
pub const LOCKBLS_ID_SIZE: usize = LockblsId::length();

// Satoshi

pub type Satoshi = Uint8;
impl Satoshi {}

// lending

pub type DiamondSyslendId = Fixed14;
pub type BitcoinSyslendId = Fixed15;
pub type UserLendingId = Fixed17;

pub const DIAMOND_SYSLEND_ID_SIZE: usize = DiamondSyslendId::length();
pub const BITCOIN_SYSLEND_ID_SIZE: usize = BitcoinSyslendId::length();
pub const USER_LENDING_ID_SIZE: usize = UserLendingId::length();

// BlockHeight Timestamp ***********

pub type BlockHeight = Uint5;
pub type Timestamp = Uint5;
pub const BLOCK_HEIGHT_SIZE: usize = BlockHeight::length();
pub const TIMESTAMP_SIZE: usize = Timestamp::length();
impl BlockHeight {}
impl Timestamp {}

// Hash ***********************

pub type Hash = Fixed32;
pub type HashHalf = Fixed16;
pub type HashNonce = Fixed8;
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
        <Fixed16 as Field>::from_u8s(pt)
    }

    pub fn get_nonce(&self) -> HashNonce {
        let pt: [u8; HASH_NONCE_SIZE] = self.bytes[0..HASH_NONCE_SIZE].try_into().unwrap();
        <Fixed8 as Field>::from_u8s(pt)
    }

}

// Diamond ***********************

pub const DIAMOND_NAME_VALID_CHARS: &[u8; 16]  = b"WTYUIAHXVMEKBSZN";
pub type DiamondName = Fixed6;
pub type DiamondNumber = Uint3;
pub type DiamondVisualGene = Fixed10;

impl DiamondName {
    pub fn name(&self) -> String {
        String::from_utf8(self.bytes.to_vec()).unwrap()
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

pub type Address = Fixed21;
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