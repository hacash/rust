
StructFieldFixedBytes!{ Fixed1 ,  1usize }
StructFieldFixedBytes!{ Fixed2 ,  2usize }
StructFieldFixedBytes!{ Fixed3 ,  3usize }
StructFieldFixedBytes!{ Fixed4 ,  4usize }
StructFieldFixedBytes!{ Fixed5 ,  5usize }
StructFieldFixedBytes!{ Fixed6 ,  6usize }
StructFieldFixedBytes!{ Fixed7 ,  7usize }
StructFieldFixedBytes!{ Fixed8 ,  8usize }
StructFieldFixedBytes!{ Fixed10, 10usize }
StructFieldFixedBytes!{ Fixed12, 12usize }
StructFieldFixedBytes!{ Fixed14, 14usize }
StructFieldFixedBytes!{ Fixed15, 15usize }
StructFieldFixedBytes!{ Fixed16, 16usize }
StructFieldFixedBytes!{ Fixed17, 17usize }
StructFieldFixedBytes!{ Fixed18, 18usize }
StructFieldFixedBytes!{ Fixed21, 21usize }
StructFieldFixedBytes!{ Fixed24, 24usize }
StructFieldFixedBytes!{ Fixed32, 32usize }
StructFieldFixedBytes!{ Fixed33, 33usize }
StructFieldFixedBytes!{ Fixed64, 64usize }



pub type StringTrim16 = Fixed16;

/* Uint

pub type Uint1 = Fixed1;
pub type Uint2 = Fixed2;
pub type Uint3 = Fixed3;
pub type Uint4 = Fixed4;
pub type Uint5 = Fixed5;
pub type Uint8 = Fixed8;
*/

// NumFloat

pub type Float4Unsafe = Fixed4;
pub type Float8Unsafe = Fixed8;

// Bool ***********************

pub type Bool = Fixed1;
impl Bool {

    pub fn set(&mut self, v: bool) {
        self.bytes[0] = match v { true => 1u8, false => 0u8 };
    }
    
    pub fn check(&self) -> bool {
        match self.to_u8() {
            0u8 => false,
            _ => true,
        }
    }

    pub fn from_bool(v: bool) -> Bool {
        let mut var = Bool::default();
        var.set(v);
        var
    }

    pub fn to_bool(&self) -> bool {
        self.check()
    }
}

// Hash ***********************

pub type Hash = Fixed32;
pub type HashHalf = Fixed16;
pub type HashNonce = Fixed8;
pub type HashCheck = Fixed4;
pub type HashMark = Fixed2;

const HASH_SIZE: usize = Hash::width();
const HASH_HALF_SIZE: usize = HashHalf::width();
const HASH_NONCE_SIZE: usize = HashNonce::width();
const HASH_CHECK_SIZE: usize = HashCheck::width();
const HASH_MARK_SIZE: usize = HashMark::width();

impl Hash {

    pub fn half(&self) -> HashHalf {
        let pt: [u8; HASH_HALF_SIZE] = self.bytes[0..HASH_HALF_SIZE].try_into().unwrap();
        HashHalf::must(&pt)
    }

    pub fn nonce(&self) -> HashNonce {
        let pt: [u8; HASH_NONCE_SIZE] = self.bytes[0..HASH_NONCE_SIZE].try_into().unwrap();
        HashNonce::must(&pt)
    }

    pub fn check(&self) -> HashCheck {
        let pt: [u8; HASH_CHECK_SIZE] = self.bytes[0..HASH_CHECK_SIZE].try_into().unwrap();
        HashCheck::must(&pt)
    }

    pub fn mark(&self) -> HashMark {
        let pt: [u8; HASH_MARK_SIZE] = self.bytes[0..HASH_MARK_SIZE].try_into().unwrap();
        HashMark::must(&pt)
    }

}
