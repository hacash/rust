

pub const AMOUNT_MIN_SIZE: usize = 2;

static _BIG_MEI_DIP_DIV: u64 = 1_00000000_00000000;
static _BIG_MEI_DIP_UNIT: u8 = 248 - 8 - 8; // 232
static _MEI_UNIT:  u8 = 248;
static _ZHU_UNIT:  u8 = 240;
static _SHUO_UNIT: u8 = 232;
static _AI_UNIT:   u8 = 224;
static _MIAO_UNIT: u8 = 216;

macro_rules! amount_check_data_len{
    ($self:expr, $tip:expr) => (
        {
            let l1 = $self.dist.abs() as usize;
            let l2 = $self.byte.len();
            if l1 != l2 {
                panic!("Amount.{}() dist abs {} is not match byte len {}.", $tip, l1, l2)
            }
        }
    )
}

#[derive(Default, Clone, Eq)]
pub struct Amount {
	unit: u8,
	dist: i8,
	byte: Vec<u8>,
}

impl fmt::Display for Amount{
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"{}",self.to_fin_string())
    }
}

impl fmt::Debug for Amount {

    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"[unit:{}, dist:{}, byte: {:?}]", self.unit, self.dist, self.byte)
    }

}

impl PartialEq for Amount {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Ord for Amount {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.equal(other) {
            return Ordering::Equal
        }
        if self.more_than(other) {
            return Ordering::Greater
        }
        return Ordering::Less
    }
}

impl PartialOrd for Amount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Mul<i32> for Amount {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        let mut v = self.to_bigint();
        v = v.mul(other);
        Amount::from_bigint(&v).unwrap()
    }
}

impl Div<i32> for Amount {
    type Output = Self;
    fn div(self, other: i32) -> Self {
        let mut v = self.to_bigint();
        v = v.div(other);
        Amount::from_bigint(&v).unwrap()
    }
}


// impl Copy for Amount {}

impl Field for Amount {

    // must & create function
    fnFieldMustCreate!(Amount);
}

impl Parse for Amount {

    fn parse(&mut self, buf: &[u8], seek: usize) -> Result<usize, Error> {
        let mut seek = seek;
        // get unit
        let btv = buf_clip_mvsk!(buf[seek..], 1);
        self.unit = btv[0];
        seek += 1;        
        // get dist
        let btv = buf_clip_mvsk!(buf[seek..], 1);
        self.dist = btv[0] as i8;
        seek += 1;
        // get bytes
        let btlen = self.dist.abs() as usize;
        let btv = buf_clip_mvsk!(buf[seek..], btlen);
        self.byte = btv;
        amount_check_data_len!(self, "parse");
        Ok(seek + btlen)
    }

}

impl Serialize for Amount {
    fn serialize(&self) -> Vec<u8> {
        let mut resv = vec!(self.unit, self.dist as u8);
        resv.append( &mut self.byte.clone() );
        resv
    }

    fn size(&self) -> usize {
        // amount_check_data_len!(self, "size");
        1 + 1 + self.dist.abs() as usize
    }

}

// new or from
impl Amount {

    pub fn new_coin(num: u8) -> Amount {
        if num % 10 == 0 {
            panic!("{} is not support.", num)
        }
        let amt = Amount{
            unit: _MEI_UNIT,
            dist: 1,
            byte: vec!(num),
        };
        amount_check_data_len!(amt, "new_coin");
        amt
    }

    pub fn new_small(num: i8, unit: u8) -> Amount {
        if num % 10 == 0 {
            panic!("{} is not support.", num)
        }
        let amt = Amount{
            unit,
            dist: match num < 0 { true => -1, false => 1 },
            byte: vec!(num.abs() as u8),
        };
        amount_check_data_len!(amt, "new_small");
        amt
    }

    pub fn from_unit_byte(unit: u8, byte: Vec<u8>) -> Result<Amount, String> {
        let bl = byte.len();
        if bl > 127 {
            return Err("amount bytes len overflow 127.".to_string())
        }
        Ok(Amount{
            unit: unit,
            dist: bl as i8,
            byte: byte,
        })
    }

    pub fn from_i64(mut num: i64, mut unit: u8) -> Result<Amount, String> {
        let mut amt = Amount::default();
        if num == 0 {
            return Ok(amt);
        }
        while num % 10 == 0 {
            num /= 10;
            unit+=1;
            if unit==255 {
                break
            }
        }
        if num % 10 == 0 {
            return errf!("Amount.from_mei_i64 `{}` format error.", num)
        }
        // parse
        let big: BigInt = FromPrimitive::from_i64(num).unwrap();
        let (sign, bigbts) = big.to_bytes_be();
        let dlen = bigbts.len();
        if dlen > 127 {
            return Err("amount is too big".to_string());
        }
        amt.unit = unit;
        let dlen = dlen as i8;
        amt.dist = match sign == Minus { true => -dlen, false => dlen };
        // byte
        amt.byte = bigbts;
        amount_check_data_len!(amt, "from_i64");
        // ok
        return Ok(amt);
    }

    pub fn from_mei(mei: i64) -> Result<Amount, String> {
        return Amount::from_i64(mei, _MEI_UNIT);
    }

    pub fn from_zhu(zhu: i64) -> Result<Amount, String> {
        return Amount::from_i64(zhu, _ZHU_UNIT);
    }

    pub fn from_shuo(shuo: i64) -> Result<Amount, String> {
        return Amount::from_i64(shuo, _SHUO_UNIT);
    }

    pub fn from_string_unsafe(v: &String) -> Result<Amount, String> {
        if let Some(t) = v.find(":") {
            Amount::from_fin_string(v)
        }else{
            Amount::from_mei_string_unsafe(v)
        }
    }

    pub fn from_mei_string_unsafe(v: &String) -> Result<Amount, String> {
        let mayerr = ||{
            errf!("Amount.from_mei_string_unsafe `{}` format error.", v)
        };
        // let mut amt = Amount::default();
        let nums: Vec<&str> = v.as_str().trim().split(".").collect();
        if 1 == nums.len() {
            // int
            let ii = match v.parse::<i64>() {
                Err(_) => return mayerr(),
                Ok(i) => i,
            };
            return Amount::from_mei(ii);

        }else if 2 == nums.len() {
            let ff = match v.parse::<f64>() {
                Err(_) => return mayerr(),
                Ok(f) => f,
            };
            // float
            let fdl = nums[1].trim_end_matches("0").len();
            if fdl > 8 + 8 {
                return Err("amount size is too big".to_string());
            }
            let base = 10i32.pow(fdl as u32) as f64;
            let ii = (ff * base) as i64;
            return Amount::from_i64(ii, _MEI_UNIT - (fdl as u8));

        }else{
            return mayerr();
        }
        
    }

    pub fn from_fin_string(v: &String) -> Result<Amount, String> {
        let v = v.to_uppercase().replace("ㄜ", " ").replace("HAC", " ");
        let v = v.as_str().trim().to_string();
        let vs:Vec<&str> = v.split(":").collect();
        let mayerr = ||{
            format!("amount fin string `{}` format error.", v)
        };
        if 2 != vs.len() {
            return Err(mayerr());
        }
        let unit_v = vs[1].to_string();
        let num_v = vs[0].to_string();
        // create amt
        let mut amt = Amount::default();
        let unit = match unit_v.parse::<u32>() {
            Ok(uv) => uv,
            Err(_) => return Err(mayerr()),
        };
        amt.unit = match unit<=255 {
            true => unit as u8,
            false => return Err(mayerr()),
        };
        // num
        // let zorebigint: BigInt = FromPrimitive::from_i8(0).unwrap();
        let bignum = match BigInt::from_str_radix(&num_v, 10) {
            Ok(n) => n,
            Err(_) => return Err(mayerr()),
        };
        let (sign, nunbts) = bignum.to_bytes_be(); // bigend
        if nunbts.len() > 127 {
            return Err(mayerr());
        }
        amt.dist = nunbts.len() as i8;
        if sign == Minus {
            amt.dist = -amt.dist; // - Minus
        }
        // println!("{} {}", bignum.to_string(), sign==Minus);
        amt.byte = nunbts;
        // check
        amount_check_data_len!(amt, "to_string");
        Ok(amt)
    }

    pub fn to_mei_unsafe(&self) -> f64 {
        self.to_unit_unsafe(_MEI_UNIT)
    }

    pub fn to_zhu_unsafe(&self) -> f64 {
        self.to_unit_unsafe(_ZHU_UNIT)
    }

    pub fn to_shuo_unsafe(&self) -> f64 {
        self.to_unit_unsafe(_SHUO_UNIT)
    }

    pub fn to_unit_unsafe(&self, base_unit: u8) -> f64 {

        // let mut amt = Amount::default();
        if self.is_empty() {
            return 0f64
        }
        let chax = (base_unit as i64 - (self.unit as i64)).abs() as u64;
        // num
        let num = BigInt::from_bytes_be(Plus, &self.byte[..]).to_f64().unwrap();
        // unit
        let base = 10f64.powf(chax as f64) as f64;
        let mut resv = match self.unit > base_unit {
            true => num * base,
            false => num / base,
        };
        // sign
        if self.dist < 0 {
            resv = resv * -1f64;
        }
        resv
    }

}

// from / to bigint 
impl Amount {

    pub fn from_bigint( bignum: &BigInt ) -> Result<Amount, String> {
        let numstr = bignum.to_string();
        if numstr == "0" {
            return Ok(Amount::default())
        }
        let numuse = numstr.as_str().trim_end_matches('0');
        let unit = numstr.len() - numuse.len();
        if unit > 255 {
            return Err("Amount is too wide.".to_string())
        }
        let biguse = BigInt::from_str_radix(&numuse, 10);
        if let Err(e) = biguse {
            return errf!("BigInt::from_str_radix error: {} {} {} {}", numstr, numuse, numuse, e.to_string())
        }
        let biguse = biguse.unwrap();
        let (sign, byte) = biguse.to_bytes_be();
        let dist = byte.len();
        if dist > 127 {
            return Err("Amount is too wide.".to_string())
        }
        let mut dist = dist as i8;
        if sign == Minus {
            dist *= -1;
        }
        // amt
        let mut amt = Amount::default();
        amt.unit = unit as u8;
        amt.dist = dist;
        amt.byte = byte;
        // check
        amount_check_data_len!(amt, "from_i64");
        // ok
        Ok(amt)
    }

    pub fn to_bigint(&self) -> BigInt {
        if self.is_empty() {
            return FromPrimitive::from_u64(0).unwrap();
        }
        let mut bignum = BigInt::from_bytes_be(Plus, &self.byte[..]);
        bignum = match self.dist < 0 {
            true => bignum * -1,
            false => bignum,
        };
        let base: BigInt = FromPrimitive::from_u64(10).unwrap();
        let powv = base.pow(self.unit as u32);
        bignum * powv
    }

}

// to string 
impl Amount {

    pub fn to_string(&self) -> String {
        ("ㄜ".to_owned() + self.to_fin_string().as_str()).to_string()
    }
    
    pub fn to_fin_string(&self) -> String {
        let (s1, s2, s3) = self.to_strings();
        format!("{}{}:{}", s1, s2, s3)
    }

    pub fn to_strings(&self) -> (String, String, String) {
        let bignum = BigInt::from_bytes_be(Plus, &self.byte[..]);
        let s1 = match self.dist < 0 {
            true => "-".to_string(),
            false => "".to_string(),
        };
        let s2 = bignum.to_string();
        let s3 = format!("{}", self.unit);
        (s1, s2, s3)
    }

    pub fn to_mei_string_unsafe(&self) -> String {
        self.to_mei_unsafe().to_string()
    }

    pub fn to_mei_or_fin_string(&self, usemei: bool) -> String {
        match usemei {
            true => self.to_mei_string_unsafe(),
            false => self.to_fin_string(),
        }
    }

    pub fn to_unit_string(&self, unit_str: &str) -> String {
        let mut unit = 0u8;
        if let Ok(u) = unit_str.parse::<u8>() {
            unit = u;
        }else{
            unit = match unit_str {
                "mei"  => _MEI_UNIT,
                "zhu"  => _ZHU_UNIT,
                "shuo" => _SHUO_UNIT,
                "ai"   => _AI_UNIT,
                "miao" => _MIAO_UNIT,
                _ => 0,
            }
        }
        if unit > 0 {
            self.to_unit_unsafe(unit).to_string()
        }else{
            self.to_fin_string()
        }
    }

}


pub trait CptMul<T> {
    fn mul(&self, val: T) -> Result<Amount, String>;
}
impl CptMul<i32> for Amount {
    fn mul(&self, val: i32) -> Result<Amount, String> {
        self.mul_u64(val as u64)
    }
}
impl CptMul<u64> for Amount {
    fn mul(&self, val: u64) -> Result<Amount, String> {
        self.mul_u64(val as u64)
    }
}
pub trait CptDiv<T> {
    fn div(&self, val: T) -> Result<Amount, String>;
}
impl CptDiv<i32> for Amount {
    fn div(&self, val: i32) -> Result<Amount, String> {
        self.div_u64(val as u64)
    }
}
impl CptDiv<u64> for Amount {
    fn div(&self, val: u64) -> Result<Amount, String> {
        self.div_u64(val as u64)
    }
}

// compute
impl Amount {

    pub fn unit(&self) -> u8 {
        self.unit
    }
    pub fn dist(&self) -> i8 {
        self.dist
    }
    pub fn byte(&self) -> &Vec<u8> {
        &self.byte
    }

    pub fn unit_sub(&mut self, v: u8) {
        if v > self.unit {
            panic!("cannot sub unit to negative number.");
        }
        self.unit -= v;
    }

    pub fn mul_u64(&self, val: u64) -> Result<Amount, String> {
        let mut v = self.to_bigint();
        v = v.mul(val);
        Amount::from_bigint(&v)
    }

    pub fn div_u64(&self, val: u64) -> Result<Amount, String> {
        let mut v = self.to_bigint();
        v = v.div(val);
        Amount::from_bigint(&v)
    }

    pub fn add(&self, amt: &Amount) -> Result<Amount, String> {
        let var1 = self.to_bigint();
        let var2 = amt.to_bigint();
        let varres = var1 + var2;
        Amount::from_bigint(&varres)
    }

    pub fn sub(&self, amt: &Amount) -> Result<Amount, String> {
        let var1 = self.to_bigint();
        let var2 = amt.to_bigint();
        let varres = var1 - var2;
        Amount::from_bigint(&varres)
    }

    pub fn compress(&self, nummaxlen: usize, upper: bool) -> Result<Amount, String> {
        let mut useamt = self.clone();
        loop {
            let (_, numstr, _) = useamt.to_strings();
            if numstr.len() <= nummaxlen {
                break; // ok
            }
            let mut nnn = numstr.parse::<u64>().unwrap();
            nnn = nnn / 10;
            let unit_n = useamt.unit as u64 + 1;
            if unit_n > 255 {
                return errf!("`{}` compress failed.", self.to_fin_string());
            }
            useamt.unit = unit_n as u8;
            if upper {
                nnn += 1;
            }
            let big_n: BigInt = FromPrimitive::from_u64(nnn).unwrap();
            (_, useamt.byte) = big_n.to_bytes_be();
            // next
        };
        Ok(useamt)
    }
}


// compare 
impl Amount {

    pub fn equal(&self, amt: &Amount) -> bool {
        if self.byte == amt.byte 
        && self.dist == amt.dist 
        && self.unit == amt.unit {
            return true
        }
        return false
    }

    pub fn not_equal(&self, amt: &Amount) -> bool {
        return self.equal(amt) == false;
    }

    pub fn more_than(&self, amt: &Amount) -> bool {
        if self.equal(amt) {
            return false
        }
        let var1 = self.to_bigint();
        let var2 = amt.to_bigint();
        match var1.cmp( &var2 ) {
            Greater => true,
            _ => false,
        }
    }

    pub fn more_or_equal(&self, amt: &Amount) -> bool {
        if self.equal(amt) {
            return true
        }
        let var1 = self.to_bigint();
        let var2 = amt.to_bigint();
        match var1.cmp( &var2 ) {
            Less => false,
            _ => true,
        }
    }

    pub fn less_than(&self, amt: &Amount) -> bool {
        if self.equal(amt) {
            return false
        }
        let var1 = self.to_bigint();
        let var2 = amt.to_bigint();
        match var1.cmp( &var2 ) {
            Less => true,
            _ => false,
        }
    }

    pub fn less_or_equal(&self, amt: &Amount) -> bool {
        if self.equal(amt) {
            return true
        }
        let var1 = self.to_bigint();
        let var2 = amt.to_bigint();
        match var1.cmp( &var2 ) {
            Greater => false,
            _ => true,
        }
    }

}

// check
impl Amount {

    pub fn is_empty(&self) -> bool {
        if self.unit == 0 {
            return true;
        }
        if self.dist == 0 {
            return true;
        }
        return false;
    }
    pub fn is_not_empty(&self) -> bool {
        return self.is_empty() == false;
    }

    // check must be positive and cannot be zero
    pub fn is_positive(&self) -> bool {
        if self.unit == 0 {
            return false
        }
        if self.dist <= 0 {
            return false
        }
        // yes
        return true
    }   

    // check must be negative and cannot be zero
    pub fn is_negative(&self) -> bool {
        if self.unit == 0 {
            return false
        }
        if self.dist >= 0 {
            return false
        }
        // yes
        return true
    }

}