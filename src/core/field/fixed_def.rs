pub const FLOAT4_SIZE: usize = 4;
pub const FLOAT8_SIZE: usize = 8;


// common fn

fn bytesfixed_to_uint(tip: &str, bts: &[u8], tsz: usize, len: usize) -> u64 {
    let sz = len;
    if sz > tsz || sz > 8 {
        panic!("{} size cannot over {}", tip, &tsz.to_string())
    }
    let mut vbts = [0u8; 8];
    let left = tsz - sz;
    let mut i = 0;
    for k in left..tsz {
        vbts[k] = bts[i];
        i += 1;
    }
    u64::from_be_bytes(vbts) 
}

fn bytesfixed_from_uint(tip: &str, val: u64, tsz: usize, len: usize) -> Vec<u8> {
    let sz = len;
    if sz > tsz {
        panic!("{} size cannot over {}", tip, &tsz.to_string())
    }
    let rlbt = val.to_be_bytes();
    let mut vbts = [0u8; 8];
    let left = tsz - sz;
    let mut i = 0;
    for k in left..tsz {
        vbts[i] = rlbt[k];
        i += 1;
    }
    vbts[0..sz].to_vec()
}

fn bytesfixed_to_readable_string(bts: &[u8]) -> String {
    let ss: Vec<u8> = bts.iter().map(|x|match x {
        32..=126 => *x,
        _ => ' ' as u8,
    }).collect();
    let resstr = String::from_utf8(ss).ok().unwrap();
    resstr.trim_end().to_string()
}

fn bytesfixed_from_readable_string(bts: &mut [u8], s: &String, len: usize) {
    let sz = len;
    let rs = s.clone().into_bytes();
    for i in 0.. s.len() {
        if i >= sz {
            break
        }
        bts[i] = rs[i];
    }
}

fn bytesfixed_from_hex(tip: &str, s: &String, len: usize) -> Result<Vec<u8>, Error> {
    let rsz = s.len();
    let tsz = len * 2;
    if rsz != tsz {
        return Err("FieldHex::".to_owned()+tip+" from_hex size error need "+&rsz.to_string()+" but got "+&rsz.to_string())
    }
    let bts = hex::decode(s);
    match bts {
        Ok(b) => Ok(b),
        Err(e) => Err(e.to_string()),
    }
}


macro_rules! bytesfixed_from_to_float_fn{
    ($tip:expr, $tarty:ident, $tsz:expr, $size:expr) => (

        concat_idents!(fn_to_1 = to_, $tarty {
        fn fn_to_1(&self) -> $tarty {
            let sz = $size;
            let tz = $tsz;
            if sz != tz {
                panic!("{} size error must be {}", $tip, tz)
            }
            <$tarty>::from_be_bytes(self.bytes[0..tz].try_into().unwrap())
        }
        });
        concat_idents!(fn_from_1 = from_, $tarty {
        fn fn_from_1(&mut self, fv: $tarty) {
            let sz = $size;
            let tz = $tsz;
            if sz != tz {
                panic!("{} size error must be {}", $tip, tz)
            }
            let bts = fv.to_be_bytes();
            self.bytes = bts[0..tz].try_into().unwrap()
        }
        });

    )
}

/******************************/


// create Fixed macro
macro_rules! create_fixed_struct_and_impl{
    ($tip:expr, $class:ident, $size:expr) => (


#[derive(Debug, Hash, Clone, PartialEq, Eq)]
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

impl_operation_for_common!($class, Add, add);
impl_operation_for_common!($class, Sub, sub);
impl_operation_for_common!($class, Mul, mul);
impl_operation_for_common!($class, Div, div);

impl_operation_for_int!($class, u8 , Add, add);
impl_operation_for_int!($class, u16, Add, add);
impl_operation_for_int!($class, u32, Add, add);
impl_operation_for_int!($class, u64, Add, add);

impl_operation_for_int!($class, u8 , Sub, sub);
impl_operation_for_int!($class, u16, Sub, sub);
impl_operation_for_int!($class, u32, Sub, sub);
impl_operation_for_int!($class, u64, Sub, sub);

impl_operation_for_int!($class, u8 , Mul, mul);
impl_operation_for_int!($class, u16, Mul, mul);
impl_operation_for_int!($class, u32, Mul, mul);
impl_operation_for_int!($class, u64, Mul, mul);

impl_operation_for_int!($class, u8 , Div, div);
impl_operation_for_int!($class, u16, Div, div);
impl_operation_for_int!($class, u32, Div, div);
impl_operation_for_int!($class, u64, Div, div);

impl_operation_for_float!($class, f32, Add, add);
impl_operation_for_float!($class, f64, Add, add);
impl_operation_for_float!($class, f32, Sub, sub);
impl_operation_for_float!($class, f64, Sub, sub);
impl_operation_for_float!($class, f32, Mul, mul);
impl_operation_for_float!($class, f64, Mul, mul);
impl_operation_for_float!($class, f32, Div, div);
impl_operation_for_float!($class, f64, Div, div);

impl_operation_assign_for_int!($class, u8 , AddAssign, add_assign, add);
impl_operation_assign_for_int!($class, u16, AddAssign, add_assign, add);
impl_operation_assign_for_int!($class, u32, AddAssign, add_assign, add);
impl_operation_assign_for_int!($class, u64, AddAssign, add_assign, add);

impl_operation_assign_for_int!($class, u8 , SubAssign, sub_assign, sub);
impl_operation_assign_for_int!($class, u16, SubAssign, sub_assign, sub);
impl_operation_assign_for_int!($class, u32, SubAssign, sub_assign, sub);
impl_operation_assign_for_int!($class, u64, SubAssign, sub_assign, sub);

impl_operation_assign_for_int!($class, u8 , MulAssign, mul_assign, mul);
impl_operation_assign_for_int!($class, u16, MulAssign, mul_assign, mul);
impl_operation_assign_for_int!($class, u32, MulAssign, mul_assign, mul);
impl_operation_assign_for_int!($class, u64, MulAssign, mul_assign, mul);

impl_operation_assign_for_int!($class, u8 , DivAssign, div_assign, div);
impl_operation_assign_for_int!($class, u16, DivAssign, div_assign, div);
impl_operation_assign_for_int!($class, u32, DivAssign, div_assign, div);
impl_operation_assign_for_int!($class, u64, DivAssign, div_assign, div);


/*
impl Add<i32> for $class {
    type Output = Self;
    #[inline]
    fn add(self, other: i32) -> Self {
        let rv = self.to_u64() + other as u64;
        <$class>::from_uint(rv)
    }
}
*/



impl Serialize for $class {

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, Error> {
        let seek2 = parse_move_seek_or_error!($tip, seek, $size, buf);
        let sv = &buf[seek..seek2];
        self.bytes = sv.try_into().unwrap();
        Ok(seek2)
    }

    fn serialize(&self) -> Vec<u8> {
        if $size != self.bytes.len() {
            panic!("serialize size not match for {} ", $tip)
        }
        self.bytes.to_vec()
    }

    fn size(&self) -> usize {
        $size
    }

}

impl Describe for $class {

    fn describe(&self) -> String {
        "".to_string()
    }

    fn to_json(&self, cnf: &FieldJsonConfig) -> String {
        "".to_string()
    }

    fn from_json(&mut self, _: &String) -> Option<Error> {
        None
    }

}

impl Field for $class {

    // parse function
    fn_field_create_by_new_wrap_return!($class);

    fn new() -> $class {
        $class{
            bytes: [0u8; $size],
        }
    }

    fn from_uint<T>(nt: T) -> Self where Self: Sized, T: std::ops::Add<u64, Output = u64> { 
        let num: u64 = nt + 0u64;
        let mut obj = <$class>::new();
        let sz = $size;
        if sz <= 1 && num < 256 {
            obj.from_u8(num as u8)
        }else if sz <= 2 && num < 65536 {
            obj.from_u16(num as u16)
        }else if sz <= 3 && num < 16777216 {
            obj.from_u32(num as u32)
        }else if sz <= 4 && num < 4294967296 {
            obj.from_u32(num as u32)
        }else if sz <= 8 {
            obj.from_u64(num as u64)
        }else {
            panic!("from_uint size cannot over 8")
        }
        obj
    }

    fn from_float<T>(nt: T) -> Self where Self: Sized, T: std::ops::Add<f64, Output = f64> {
        let mut obj = <$class>::new();
        let sz = $size;
        if sz != 4 && sz != 8 {
            panic!("{} from_float size error must be 4 or 8 but got {}", $tip, sz)
        }
        let num: f64 = nt + 0f64;
        if sz == 4 {
            obj.from_f32(num as f32)
        }else{
            obj.from_f64(num as f64)
        }
        obj
    }

    fn from_u8s(buf: impl AsRef<[u8]>) -> Self where Self: Sized {
        let v = buf.as_ref().clone();
        if v.len() != $size {
            panic!("size error")
        }
        // obj
        let mut obj = <$class>::new();
        obj.bytes = v.try_into().unwrap();
        // ok
        obj
    }

}

impl FieldHex for $class {

    fn to_hex(&self) -> String {
        hex::encode(self.bytes)
    }

    fn from_hex(&mut self, s: &String) -> Option<Error> {
        let res = bytesfixed_from_hex($tip, s, $size);
        let bts = match res {
            Ok(b) => b,
            Err(e) => return Some(e),
        };
        self.bytes = bts.try_into().unwrap();
        None
    }

}

impl FieldBytes for $class {

    fn to_vec_u8(&self) -> Vec<u8> {
        self.serialize()
    }

    fn from_vec_u8(&mut self, buf: &Vec<u8>) -> Option<Error> {
        self.parse(buf, 0).err()
    }

}

impl FieldNumber for $class {

    fn to_u8(&self) -> u8 {
        let rv = bytesfixed_to_uint($tip, &self.bytes[..], 1, $size);
        rv as u8
    }

    fn from_u8(&mut self, val: u8) {
        let bts = bytesfixed_from_uint($tip, val as u64, 1, $size);
        self.bytes = bts.try_into().unwrap();
    }

    fn to_u16(&self) -> u16 {
        let rv = bytesfixed_to_uint($tip, &self.bytes[..], 2, $size);
        rv as u16
    }

    fn from_u16(&mut self, val: u16) {
        let bts = bytesfixed_from_uint($tip, val as u64, 2, $size);
        self.bytes = bts.try_into().unwrap();
    }

    fn to_u32(&self) -> u32 {
        let rv = bytesfixed_to_uint($tip, &self.bytes[..], 4, $size);
        rv as u32
    }

    fn from_u32(&mut self, val: u32) {
        let bts = bytesfixed_from_uint($tip, val as u64, 4, $size);
        self.bytes = bts.try_into().unwrap();
    }

    fn to_u64(&self) -> u64 {
        bytesfixed_to_uint($tip, &self.bytes[..], 8, $size)
    }

    fn from_u64(&mut self, val: u64) {
        let bts = bytesfixed_from_uint($tip, val, 8, $size);
        self.bytes = bts.try_into().unwrap();
    }

    bytesfixed_from_to_float_fn!($tip, f32, 4, $size);
    bytesfixed_from_to_float_fn!($tip, f64, 8, $size);

}

impl FieldReadableString for $class {
    
    fn to_readable_string(&self) -> String {
        bytesfixed_to_readable_string(&self.bytes[..])
    }
    
    fn from_readable_string(&mut self, s: &String) -> Option<Error> {
        bytesfixed_from_readable_string(&mut self.bytes[..], s, $size);
        None
    }

}

impl $class {

    const fn length() -> usize {
        $size
    }

}





    )
}



// create_fixed_struct_and_impl!("Fixed4 ", Fixed4 ,  4usize);


// create 
create_fixed_struct_and_impl!("Fixed1 ", Fixed1 ,  1usize);
create_fixed_struct_and_impl!("Fixed2 ", Fixed2 ,  2usize);
create_fixed_struct_and_impl!("Fixed3 ", Fixed3 ,  3usize);
create_fixed_struct_and_impl!("Fixed4 ", Fixed4 ,  4usize);
create_fixed_struct_and_impl!("Fixed5 ", Fixed5 ,  5usize);
create_fixed_struct_and_impl!("Fixed6 ", Fixed6 ,  6usize);
create_fixed_struct_and_impl!("Fixed8 ", Fixed8 ,  8usize);
create_fixed_struct_and_impl!("Fixed10", Fixed10, 10usize);
create_fixed_struct_and_impl!("Fixed12", Fixed12, 12usize);
create_fixed_struct_and_impl!("Fixed14", Fixed14, 14usize);
create_fixed_struct_and_impl!("Fixed15", Fixed15, 15usize);
create_fixed_struct_and_impl!("Fixed16", Fixed16, 16usize);
create_fixed_struct_and_impl!("Fixed17", Fixed17, 17usize);
create_fixed_struct_and_impl!("Fixed18", Fixed18, 18usize);
create_fixed_struct_and_impl!("Fixed21", Fixed21, 21usize);
create_fixed_struct_and_impl!("Fixed24", Fixed24, 24usize);
create_fixed_struct_and_impl!("Fixed32", Fixed32, 32usize);
create_fixed_struct_and_impl!("Fixed33", Fixed33, 33usize);
create_fixed_struct_and_impl!("Fixed64", Fixed64, 64usize);
