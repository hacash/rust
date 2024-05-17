
const AUTOU64SX0: u64 = 128; // 1 : 128
const AUTOU64SX1: u64 = 64 * 256; // 2 : 16384
const AUTOU64SX2: u64 = 32 * 256 * 256; // 3 : 2097152
const AUTOU64SX3: u64 = 16 * 256 * 256 * 256; // 4 : 2_68435456
const AUTOU64SX4: u64 = 8  * 256 * 256 * 256 * 256; // 5 : 343_59738368
const AUTOU64SX5: u64 = 4  * 256 * 256 * 256 * 256 * 256; // 6 : 43980_46511104
const AUTOU64SX6: u64 = 2  * 256 * 256 * 256 * 256 * 256 * 256; // 7 : 5629499_53421312
const AUTOU64SX7: u64 = 1  * 256 * 256 * 256 * 256 * 256 * 256 * 256; // 8 : 7_20575940_37927936
// MAX                  1  * 256 * 256 * 256 * 256 * 256 * 256 * 256 * 256; // 9 : 1844_67440737_09551616
pub const AUTOU64XLIST: [u64; 8] = [AUTOU64SX0, AUTOU64SX1, AUTOU64SX2, AUTOU64SX3, AUTOU64SX4, AUTOU64SX5, AUTOU64SX6, AUTOU64SX7];
const AUTOU64MATCH: [u8;  8] = [0b00000000, 0b10000000, 0b11000000, 0b11100000, 0b11110000, 0b11111000, 0b11111100, 0b11111110];

macro_rules! ckprfxn{
    ( $a:ident, $n:ident, $bv:expr, $bm:expr ) => (
        if $a & $bv == $bv {
            $n += 1;
        }else{
            return ($n, $a & $bm);
        }
    )
}

fn ckprefix1n(a: u8) -> (u8, u8) {
    let mut n = 0;
    ckprfxn!{a, n, 0b10000000, 0b01111111}
    ckprfxn!{a, n, 0b01000000, 0b00111111}
    ckprfxn!{a, n, 0b00100000, 0b00011111}
    ckprfxn!{a, n, 0b00010000, 0b00001111}
    ckprfxn!{a, n, 0b00001000, 0b00000111}
    ckprfxn!{a, n, 0b00000100, 0b00000011}
    ckprfxn!{a, n, 0b00000010, 0b00000001}
    ckprfxn!{a, n, 0b00000001, 0b00000000}
    (8, 0)
}



#[derive(Default, Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct AutoU64 {
    value: u64,
}

impl std::fmt::Display for AutoU64{
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f,"{}", self.value)
    }
}


impl Deref for AutoU64 {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.value
    }
}

impl PartialOrd for AutoU64 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AutoU64 {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

uintDefineAllOperate!{ AutoU64, u64, 
    u8, i8,
    u16, i16,
    u32, i32,
    u64, i64,
    usize, isize
}


impl Add for AutoU64 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {value: self.value + other.value}
    }
}

impl Sub for AutoU64 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {value: self.value - other.value}
    }
}

impl Mul for AutoU64 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {value: self.value * other.value}
    }
}

impl Div for AutoU64 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        Self {value: self.value / other.value}
    }
}

impl AddAssign for AutoU64 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
    }
}

impl SubAssign for AutoU64 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.value;
    }
}

impl MulAssign for AutoU64 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.value *= other.value;
    }
}

impl DivAssign for AutoU64 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        self.value /= other.value;
    }
}

impl Parse for AutoU64 {

    fn parse(&mut self, buf: &[u8], seek: usize) -> Ret<usize> {
        let mut sk = seek;
        let head = buf_clip_mvsk!(buf[seek..], 1);
        sk += 1;
        let (addbyte, b1v) = ckprefix1n(head[0]);
        let bdsz = addbyte as usize;
        let mut body = buf_clip_mvsk!(buf[sk..], bdsz);
        sk += bdsz;
        if bdsz < 8 {
            body = [vec![b1v], body.to_vec()].concat();
        }
        if body.len() < 8 {
            body = [vec![0u8; 8-body.len()], body.to_vec()].concat();
        }
        self.value = u64::from_be_bytes(body.try_into().unwrap());
        Ok(sk)
    }

}


impl Serialize for AutoU64 {

    fn serialize(&self) -> Vec<u8> {
        let mut apsz = 0;
        let mut nvbt = self.value.to_be_bytes().to_vec();
        nvbt = vec![vec![0b00000000], nvbt].concat(); // 9byte
        for i in 0..AUTOU64XLIST.len() {
            let x = AUTOU64XLIST[i];
            if self.value < x {
                break
            }else{
                apsz += 1;
            }
        }
        if apsz >= 8 {
            nvbt[0] |= 0b11111111;
            return nvbt.to_vec()
        }
        let mchdn = AUTOU64MATCH[apsz];
        let mut resbt = &mut nvbt[9-(apsz+1)..];
        resbt[0] |= mchdn;
        resbt.to_vec()
    }

    fn size(&self) -> usize {
        let mut sz = 1;
        for i in 0..AUTOU64XLIST.len() {
            let x = AUTOU64XLIST[i];
            if self.value < x {
                break
            }else{
                sz += 1;
            }
        }
        sz
    }

}

impl Field for AutoU64 {

    // must & create function
    fnFieldMustCreate!(AutoU64);


    fn from_uint<T>(nt: T) -> Self where Self: Sized, T: std::ops::Add<u64, Output = u64> { 
        AutoU64::from(nt + 0u64)
    }


}



impl AutoU64 {

    pub fn from(v: u64) -> AutoU64 {
        AutoU64{
            value: v,
        }
    }

    // maybe panic
    pub fn from_bytes(buf: &[u8]) -> AutoU64 {
        let mut v = AutoU64::default();
        v.parse(buf, 0).unwrap();
        v
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.serialize()
    }
    
    pub fn uint(&self) -> u64 {
        self.value
    }

    pub fn value(&self) -> u64 {
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
    pub fn add(&self, v: u64) -> AutoU64 {
        AutoU64 {
            value: self.value + v,
        }
    }
    pub fn sub(&self, v: u64) -> AutoU64 {
        AutoU64 {
            value: self.value - v,
        }
    }
    pub fn mul(&self, v: u64) -> AutoU64 {
        AutoU64 {
            value: self.value * v,
        }
    }
    pub fn div(&self, v: u64) -> AutoU64 {
        AutoU64 {
            value: self.value / v,
        }
    }
}


