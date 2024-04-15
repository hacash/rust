
// create Fixed macro
macro_rules! StructFieldFixedBytes{
    ($class:ident, $size:expr) => (


#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct $class {
    bytes: [u8; $size],
}


impl fmt::Display for $class{
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"{}",self.to_hex())
    }
}

impl Index<usize> for $class {
    type Output = u8;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.bytes[idx]
    }
}

impl IndexMut<usize> for $class {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output{
        &mut self.bytes[idx]
    }
}

impl Deref for $class {
    type Target = [u8; $size];
    fn deref(&self) -> &[u8; $size] {
        &self.bytes
    }
}

impl AsRef<[u8]> for $class {
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}

implComputeTraitCommon!($class, Add, add);
implComputeTraitCommon!($class, Sub, sub);
implComputeTraitCommon!($class, Mul, mul);
implComputeTraitCommon!($class, Div, div);

implComputeTraitInt!($class, u8 , Add, add);
implComputeTraitInt!($class, u16, Add, add);
implComputeTraitInt!($class, u32, Add, add);
implComputeTraitInt!($class, u64, Add, add);

implComputeTraitInt!($class, u8 , Sub, sub);
implComputeTraitInt!($class, u16, Sub, sub);
implComputeTraitInt!($class, u32, Sub, sub);
implComputeTraitInt!($class, u64, Sub, sub);

implComputeTraitInt!($class, u8 , Mul, mul);
implComputeTraitInt!($class, u16, Mul, mul);
implComputeTraitInt!($class, u32, Mul, mul);
implComputeTraitInt!($class, u64, Mul, mul);

implComputeTraitInt!($class, u8 , Div, div);
implComputeTraitInt!($class, u16, Div, div);
implComputeTraitInt!($class, u32, Div, div);
implComputeTraitInt!($class, u64, Div, div);

implComputeTraitFloat!($class, f32, Add, add);
implComputeTraitFloat!($class, f64, Add, add);
implComputeTraitFloat!($class, f32, Sub, sub);
implComputeTraitFloat!($class, f64, Sub, sub);
implComputeTraitFloat!($class, f32, Mul, mul);
implComputeTraitFloat!($class, f64, Mul, mul);
implComputeTraitFloat!($class, f32, Div, div);
implComputeTraitFloat!($class, f64, Div, div);

implComputeAssignTraitInt!($class, u8 , AddAssign, add_assign, add);
implComputeAssignTraitInt!($class, u16, AddAssign, add_assign, add);
implComputeAssignTraitInt!($class, u32, AddAssign, add_assign, add);
implComputeAssignTraitInt!($class, u64, AddAssign, add_assign, add);

implComputeAssignTraitInt!($class, u8 , SubAssign, sub_assign, sub);
implComputeAssignTraitInt!($class, u16, SubAssign, sub_assign, sub);
implComputeAssignTraitInt!($class, u32, SubAssign, sub_assign, sub);
implComputeAssignTraitInt!($class, u64, SubAssign, sub_assign, sub);

implComputeAssignTraitInt!($class, u8 , MulAssign, mul_assign, mul);
implComputeAssignTraitInt!($class, u16, MulAssign, mul_assign, mul);
implComputeAssignTraitInt!($class, u32, MulAssign, mul_assign, mul);
implComputeAssignTraitInt!($class, u64, MulAssign, mul_assign, mul);

implComputeAssignTraitInt!($class, u8 , DivAssign, div_assign, div);
implComputeAssignTraitInt!($class, u16, DivAssign, div_assign, div);
implComputeAssignTraitInt!($class, u32, DivAssign, div_assign, div);
implComputeAssignTraitInt!($class, u64, DivAssign, div_assign, div);


/*
impl Add<i32> for $class {
    type Output = Self;
    #[inline]
    fn add(self, other: i32) -> Self {
        let rv = self.to_u64() + other as u64;
        <$class>::from(rv)
    }
}
*/


impl Serialize for $class {


    fn serialize(&self) -> Vec<u8> {
        if $size != self.bytes.len() {
            panic!("serialize size not match")
        }
        self.bytes.to_vec()
    }

    fn size(&self) -> usize {
        $size
    }

}


impl Parse for $class {

    fn parse(&mut self, buf: &[u8], seek: usize) -> Ret<usize> {
        let bts = buf_clip_mvsk!(buf[seek..], $size);
        self.bytes = bts.try_into().unwrap();
        Ok(seek + $size)
    }


}


impl Field for $class {

    fn new() -> $class {
        $class{
            bytes: [0u8; $size],
        }
    }

    // must & create function
    fnFieldMustCreate!($class);

    fn from_uint<T>(nt: T) -> Self where Self: Sized, T: std::ops::Add<u64, Output = u64> { 
        let mut obj = <$class>::new();
        // obj.parse_uint(nt).unwrap();
        field_parse_uint(&mut obj, nt, $size).unwrap();
        obj
    }

    /*
    fn parse_uint<T>(&mut self, nt: T) -> Option<Error> where T: std::ops::Add<u64, Output = u64> { 
        let num: u64 = nt + 0u64;
        let sz = $size;
        let err = Some(s!("from_uint size error or cannot over 8"));
        match sz {
            1 => if num <= UINT_MAX_W1 { self.parse_u8(  num as u8  ) }else{ err },
            2 => if num <= UINT_MAX_W2 { self.parse_u16( num as u16 ) }else{ err },
            3 => if num <= UINT_MAX_W3 { self.parse_u32( num as u32 ) }else{ err },
            4 => if num <= UINT_MAX_W4 { self.parse_u32( num as u32 ) }else{ err },
            5 => if num <= UINT_MAX_W5 { self.parse_u64( num as u64 ) }else{ err },
            6 => if num <= UINT_MAX_W6 { self.parse_u64( num as u64 ) }else{ err },
            7 => if num <= UINT_MAX_W7 { self.parse_u64( num as u64 ) }else{ err },
            8 => if num <= UINT_MAX_W8 { self.parse_u64( num as u64 ) }else{ err },
            _ => err,
        }
    }
    */

    fn from_float<T>(nt: T) -> Self where Self: Sized, T: std::ops::Add<f64, Output = f64> {
        let mut obj = <$class>::new();
        // obj.parse_float(nt).unwrap();
        field_parse_float(&mut obj, nt, $size).unwrap();
        obj
    }

    /*
    fn parse_float<T>(&mut self, nt: T) -> Option<Error> where T: std::ops::Add<f64, Output = f64> {
        let num: f64 = nt + 0f64;
        let sz = $size;
        match sz {
            4 => self.parse_f32(num as f32),
            8 => self.parse_f64(num as f64),
            _ => Some(s!("from_float size error must be 4 or 8")),
        }
    }
    */

}

impl FieldHex for $class {

    fn to_hex(&self) -> String {
        hex::encode(self.bytes)
    }

    fn from_hex(stuff: &[u8]) -> Self where Self: Sized {
        let bts = bytes_from_hex(stuff, $size).unwrap();
        let mut obj = <$class>::new();
        obj.bytes = bts.try_into().unwrap();
        obj
    }

    /*
    fn create_by_hex(buf: &[u8]) -> Result<(Self, usize), Error> where Self: Sized { 
    }
    */


}

/*
impl FieldBytes for $class {

    fn to_vec_u8(&self) -> Vec<u8> {
        self.serialize()
    }

    fn from_vec_u8(&mut self, buf: &Vec<u8>) -> Option<Error> {
        self.parse(buf, 0).err()
    }

}
*/

impl FieldUint for $class {

    fnUintFromToParseBytes!($class, u8, 1, $size);
    fnUintFromToParseBytes!($class, u16, 2, $size);
    fnUintFromToParseBytes!($class, u32, 4, $size);
    fnUintFromToParseBytes!($class, u64, 8, $size);
    // unsafe
    fnUintFromToParseBytes!($class, usize, USIZE_WIDTH as usize, $size);
}


impl FieldFloat for $class {

    // fn from_f32 from_f64 to_f32 to_f64
    fnFloatFromToParseBytes!($class, f32, 4, $size);
    fnFloatFromToParseBytes!($class, f64, 8, $size);

}

impl FieldReadable for $class {
    
    fn to_readable(&self) -> String {
        bytes_to_readable_string(&self.bytes[..])
    }
    
    fn from_readable(stuff: &[u8]) -> Self where Self: Sized {
        let bts = bytes_from_readable_string(stuff, $size).unwrap();
        let mut obj = <$class>::new();
        obj.bytes = bts.try_into().unwrap();
        obj
    }

}

impl $class {

    pub const fn max() -> u64 { 
        let sz = $size;
        let ml = UINT_MAX_DEFS.len();
        if sz + 1 > ml {
            panic_never_call_this!();
            return 0;
        }
        let idx = if sz + 1 > ml { ml - 1 }else { sz };
        UINT_MAX_DEFS[idx] // maybe panic!
    }

    pub const fn width() -> usize {
        $size
    }

    pub const fn cons(v: [u8; $size]) -> $class {
        $class {
            bytes: v,
        }
    }

    pub fn is_zero(&self) -> bool {
        return ! self.is_not_zero()
    }

    pub fn is_not_zero(&self) -> bool {
        for a in self.bytes {
            if a > 0 {
                return true
            }
        }
        return false
    }

}





    )
}



// test
// StructFieldFixedBytes!{ Fixed1 ,  1usize }
// pub type Uint1 = Fixed1;
