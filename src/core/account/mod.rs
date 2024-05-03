use base58check::*;
use libsecp256k1::{ util, SecretKey, PublicKey, Signature, Message };
// use rand::{self, RngCore};

use crate::sys::*;
use crate::x16rs::{ sha2, ripemd160 };

// static max_prikey_value: [u8; 32] = b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364140";

const ADDRESS_SIZE: usize = 21;


#[derive(Clone, PartialEq)]
pub struct Account {
    secret_key: SecretKey,
    public_key: PublicKey,
    address: [u8; ADDRESS_SIZE],
    address_readable: String,
}



impl Account {
    pub fn secret_key(&self) -> &SecretKey {
        &self.secret_key
    }
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }
    pub fn address(&self) -> &[u8; ADDRESS_SIZE] {
        &self.address
    }
    pub fn readable(&self) -> &String {
        &self.address_readable
    }
}


include!("acc.rs");
include!("sign.rs");

