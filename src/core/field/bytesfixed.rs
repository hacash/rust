// common fn

fn bytesfixed_to_u64(tip: &str, bts: &[u8], len: usize) -> u64 {
    let sz = len;
    if sz > 8 {
        panic!(tip.to_owned()+" size cannot over 8 for to_u64.")
    }
    let mut vbts = [0u8; 8];
    let left = 8 - sz;
    let mut i = 0;
    for k in left..8 {
        vbts[k] = bts[i];
        i += 1;
    }
    u64::from_be_bytes(vbts) 
}

fn bytesfixed_from_u64(tip: &str, val: &u64, len: usize) -> Vec<u8> {
    let sz = len;
    if sz > 8 {
        panic!(tip.to_owned()+" size cannot over 8 for from_u64.")
    }
    let rlbt = val.to_be_bytes();
    let mut vbts = [0u8; 8];
    let left = 8 - sz;
    let mut i = 0;
    for k in left..8 {
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

/******************************/


// create BytesFixed macro
macro_rules! create_bytesfixed_struct_and_impl{
    ($tip:expr, $name:ident, $size:expr) => (


#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct $name {
    bytes: [u8; $size],
}


impl Serialize for $name {

     fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, Error> {
        let seek2 = parse_move_seek_or_buf_too_short_error!($tip, seek, $size, buf);
        let sv = &buf[seek..seek2];
        self.bytes = sv.try_into().unwrap();
        Ok(seek2)
    }

     fn serialize(&self) -> Vec<u8> {
        if $size != self.bytes.len() {
            panic!($tip.to_owned()+" serialize size not match.")
        }
        self.bytes.to_vec()
    }

     fn size(&self) -> usize {
        $size
    }

}

impl Describe for $name {

    fn describe(&self) -> String {
        "".to_string()
    }

    fn to_json(&self) -> String {
        "".to_string()
    }

    fn from_json(&mut self, _: &String) -> Option<Error> {
        None
    }

}

impl Field for $name {

    fn new() -> $name {
        $name{
            bytes: [0u8; $size],
        }
    }

}

impl FieldHex for $name {

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

impl FieldBytes for $name {

    fn to_vec_u8(&self) -> Vec<u8> {
        self.serialize()
    }

    fn from_vec_u8(&mut self, buf: &Vec<u8>) -> Option<Error> {
        self.parse(buf, 0).err()
    }

}

impl FieldNumber for $name {

    fn to_u64(&self) -> u64 {
        bytesfixed_to_u64($tip, &self.bytes[..], $size)
    }

    fn from_u64(&mut self, val: &u64) {
        let bts = bytesfixed_from_u64($tip, val, $size);
        self.bytes = bts.try_into().unwrap();
    }

}

impl FieldReadableString for $name {
    
    fn to_string(&self) -> String {
        bytesfixed_to_readable_string(&self.bytes[..])
    }
    
    fn from_string(&mut self, s: &String) -> Option<Error> {
        bytesfixed_from_readable_string(&mut self.bytes[..], s, $size);
        None
    }

}




impl $name {



}





    )
}



// create_bytesfixed_struct_and_impl!("Fixedbytes4 ", Fixedbytes4 ,  4usize);


// create 
create_bytesfixed_struct_and_impl!("Fixedbytes1 ", Fixedbytes1 ,  1usize);
create_bytesfixed_struct_and_impl!("Fixedbytes2 ", Fixedbytes2 ,  2usize);
create_bytesfixed_struct_and_impl!("Fixedbytes3 ", Fixedbytes3 ,  3usize);
create_bytesfixed_struct_and_impl!("Fixedbytes4 ", Fixedbytes4 ,  4usize);
create_bytesfixed_struct_and_impl!("Fixedbytes5 ", Fixedbytes5 ,  5usize);
create_bytesfixed_struct_and_impl!("Fixedbytes6 ", Fixedbytes6 ,  6usize);
create_bytesfixed_struct_and_impl!("Fixedbytes8 ", Fixedbytes8 ,  8usize);
create_bytesfixed_struct_and_impl!("Fixedbytes10", Fixedbytes10, 10usize);
create_bytesfixed_struct_and_impl!("Fixedbytes12", Fixedbytes12, 12usize);
create_bytesfixed_struct_and_impl!("Fixedbytes14", Fixedbytes14, 14usize);
create_bytesfixed_struct_and_impl!("Fixedbytes15", Fixedbytes15, 15usize);
create_bytesfixed_struct_and_impl!("Fixedbytes16", Fixedbytes16, 16usize);
create_bytesfixed_struct_and_impl!("Fixedbytes17", Fixedbytes17, 17usize);
create_bytesfixed_struct_and_impl!("Fixedbytes18", Fixedbytes18, 18usize);
create_bytesfixed_struct_and_impl!("Fixedbytes21", Fixedbytes21, 21usize);
create_bytesfixed_struct_and_impl!("Fixedbytes24", Fixedbytes24, 24usize);
create_bytesfixed_struct_and_impl!("Fixedbytes32", Fixedbytes32, 32usize);
create_bytesfixed_struct_and_impl!("Fixedbytes33", Fixedbytes33, 33usize);
create_bytesfixed_struct_and_impl!("Fixedbytes64", Fixedbytes64, 64usize);
