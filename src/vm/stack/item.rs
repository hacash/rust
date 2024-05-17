
#[derive(Debug, Clone)]
pub enum ValueItem {
    Abort(String),
    
    Nil,
    Bool(bool),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),

    Address([u8; 21]),  
    Hash([u8; 32]),  

    Buffer(Vec<u8>),
}

use ValueItem::*;

impl ValueItem {

    pub fn val_size(&self) -> usize {
        match self {
            Bool(_) => 1,
            U8(_) => 1,
            U16(_) => 2,
            U32(_) => 4,
            U64(_) => 8,
            U128(_) => 16,
            Address(_) => 21,
            Hash(_) => 32,
            Buffer(b) => b.len(),
            _ => 0, 
        }
    }

    pub fn cast_bool(&mut self) -> RetErr {
        let bv = match self {
            Nil => false,
            Bool(b) => *b,
            U8(n)   => *n != 0,
            U16(n)  => *n != 0,
            U32(n)  => *n != 0,
            U64(n)  => *n != 0,
            U128(n) => *n != 0,
            Address(b) => buf_is_not_zero(b),
            Hash(b)    => buf_is_not_zero(b),
            Buffer(b)  => buf_is_not_zero(b),
            s => return errf!("cannot cast {:?} to bool", s),
        };
        // update
        *self = Bool(bv);
        Ok(())
    }

}





/**
* ret: change left(-1) nothing(0) or right(1), Cannot do cast
*/
pub fn castv(l: &mut ValueItem, r: &mut ValueItem) -> Ret<i8> {
    match (l, r) {
        (Bool(_),   Bool(_))   => Ok(0),
        (U8(_),     U8(_))     => Ok(0),
        (U16(_),    U16(_))    => Ok(0),
        (U32(_),    U32(_))    => Ok(0),
        (U64(_),    U64(_))    => Ok(0),
        (U128(_),   U128(_))   => Ok(0),
        (Buffer(_), Buffer(_)) => Ok(0),

        (l, Bool(_)) => { l.cast_bool()?; Ok(-1) },
        (Bool(_), r) => { r.cast_bool()?; Ok(1) },

        (l, r) => errf!("cannot do cast between with type {:?} and {:?}", l, r),
    }
}

