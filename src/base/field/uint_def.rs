
// PartialEq
macro_rules! uintDefinePartial{
    ($name:ident, $vty:ty, $eqty:ty) => (
        impl PartialEq<$eqty>  for $name {
            #[inline]
            fn eq(&self, other: &$eqty) -> bool {
                self.value == *other as $vty
            }
        }
        impl PartialOrd<$eqty> for $name {
            #[inline]
            fn partial_cmp(&self, other: &$eqty) -> Option<Ordering> {
                let v = *other as $vty;
                Some(self.value.cmp(&v))
            }
        }
    )
}


macro_rules! uintDefineCompute{
    ($name:ident, $vty:ty, $eqty:ty) => (   
        impl Add<$eqty> for $name {
            type Output = Self;
            #[inline]
            fn add(self, other: $eqty) -> Self {
                Self {value: self.value + (other as $vty)}
            }
        }
        impl AddAssign<$eqty> for $name {
            #[inline]
            fn add_assign(&mut self, other: $eqty) {
                self.value += other as $vty;
            }
        }
        impl Sub<$eqty> for $name {
            type Output = Self;
            #[inline]
            fn sub(self, other: $eqty) -> Self {
                Self {value: self.value - (other as $vty)}
            }
        }
        impl SubAssign<$eqty> for $name {
            #[inline]
            fn sub_assign(&mut self, other: $eqty) {
                self.value -= other as $vty;
            }
        }
        impl Mul<$eqty> for $name {
            type Output = Self;
            #[inline]
            fn mul(self, other: $eqty) -> Self {
                Self {value: self.value * other as $vty}
            }
        }
        impl MulAssign<$eqty> for $name {
            #[inline]
            fn mul_assign(&mut self, other: $eqty) {
                self.value *= other as $vty;
            }
        }
        impl Div<$eqty> for $name {
            type Output = Self;
            #[inline]
            fn div(self, other: $eqty) -> Self {
                Self {value: self.value / other as $vty}
            }
        }
        impl DivAssign<$eqty> for $name {
            #[inline]
            fn div_assign(&mut self, other: $eqty) {
                self.value /= other as $vty;
            }
        }
    )
}


macro_rules! uintDefineAllOperate{
    ($name:ident, $vty:ty, $( $opty:ty ),+ ) => (
        $(
            uintDefinePartial!{$name, $vty, $opty}
            uintDefineCompute!{$name, $vty, $opty}
        )+
    )
}


/////////////////////////////////

macro_rules! StructFieldUint{
    ($name:ident, $vty:ty, $size:expr, $size_vl:expr) => (

#[derive(Default, Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct $name {
    value: $vty,
}

impl std::fmt::Display for $name{
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f,"{}",self.value)
    }
}

impl Deref for $name {
    type Target = $vty;
    fn deref(&self) -> &$vty {
        &self.value
    }
}

impl PartialOrd for $name {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for $name {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}


uintDefineAllOperate!{ $name, $vty, 
    u8, i8,
    u16, i16,
    u32, i32,
    u64, i64,
    usize, isize
}


impl Add for $name {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {value: self.value + other.value}
    }
}

impl Sub for $name {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {value: self.value - other.value}
    }
}

impl Mul for $name {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {value: self.value * other.value}
    }
}

impl Div for $name {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        Self {value: self.value / other.value}
    }
}

impl AddAssign for $name {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
    }
}

impl SubAssign for $name {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.value;
    }
}

impl MulAssign for $name {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.value *= other.value;
    }
}

impl DivAssign for $name {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        self.value /= other.value;
    }
}


impl Parse for $name {

    fn parse(&mut self, buf: &[u8], seek: usize) -> Ret<usize> {
        let bts = buf_clip_mvsk!(buf[seek..], $size);
        self.value = <$name>::from_bytes_value(bts.try_into().unwrap());
        Ok(seek + $size)
    }

}


impl Serialize for $name {


    fn serialize(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }


    fn size(&self) -> usize {
        <$name>::width()
    }


}

impl Field for $name {

    // must & create function
    fnFieldMustCreate!($name);

    fn from_uint<T>(nt: T) -> Self where Self: Sized, T: std::ops::Add<u64, Output = u64> { 
        <$name>::from_u64(nt + 0u64)
    }


}

impl FieldReadable for $name {
    fn readable(&self) -> String {
        format!("{}", self.value)
    }
}


/*
impl FieldNumber for $name {

    fn get_value(&self) -> u64 {
        self.value() as u64
    }

    fn set_value(&mut self, v: u64) {
        self.value = v as $vty;
    }
}
*/

impl $name {


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

    pub const fn from(v: $vty) -> $name {
        $name{
            value: v,
        }
    }

    pub fn from_u64(v: u64) -> $name {
        <$name>::from(v as $vty)
    }

    pub fn from_string(strv: &String) -> Result<$name, String> {
        match strv.parse::<u64>() {
            Err(e) => Err(e.to_string()),
            Ok(v) => Ok(<$name>::from(v as $vty)),
        }
    }

    pub fn from_bytes_value(v: [u8; $size]) -> $vty {
        // add left zore
        let drop_zore = $size_vl - $size;
        let mut rv = Vec::with_capacity($size_vl);
        rv.resize(drop_zore, 0u8);
        let appbts = &mut v.to_vec();
        rv.append(appbts);
        <$vty>::from_be_bytes(rv.try_into().unwrap())
    }

    pub fn from_bytes(v: [u8; $size]) -> $name {
        $name{
            value: <$name>::from_bytes_value(v),
        }
    }

    pub fn to_bytes(&self) -> [u8; $size] {
        let mut real = [0u8; $size];
        let bts = <$vty>::to_be_bytes(self.value);
        for x in 1..=$size {
            real[$size-x] = bts[$size_vl-x];
        }
        // println!("Uint to_bytes size {} bts {} real {}", $size, hex::encode(bts), hex::encode(real));
        real
    }
    
    pub fn uint(&self) -> $vty {
        self.value
    }

    pub fn value(&self) -> $vty {
        self.value
    }

    pub fn to_usize(&self) -> usize {
        self.value as usize
    }

    pub fn to_u8(&self) -> u8 {
        self.value as u8
    }

    pub fn to_u16(&self) -> u16 {
        self.value as u16
    }

    pub fn to_u32(&self) -> u32 {
        self.value as u32
    }

    pub fn to_u64(&self) -> u64 {
        self.value as u64
    }

    // add sub mul div
    pub fn add(&self, v: $vty) -> $name {
        $name {
            value: self.value + v,
        }
    }
    pub fn sub(&self, v: $vty) -> $name {
        $name {
            value: self.value - v,
        }
    }
    pub fn mul(&self, v: $vty) -> $name {
        $name {
            value: self.value * v,
        }
    }
    pub fn div(&self, v: $vty) -> $name {
        $name {
            value: self.value / v,
        }
    }
}




    )
}



