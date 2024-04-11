

// create Bytes macro
macro_rules! StructFieldBytes{
    ($class:ident, $lenty:ty, $size_max:expr) => (

#[derive(Debug, Clone, Eq)]
pub struct $class {
    len: $lenty,
    bytes: Vec<u8>,
}

impl PartialEq for $class {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}


impl Index<usize> for $class {
    type Output = u8;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.bytes[idx]
    }
}

impl AsRef<[u8]> for $class {
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}



impl Serialize for $class {

    fn serialize(&self) -> Vec<u8> {
        let lv = self.size();
        let mut res = Vec::with_capacity(lv);
        if self.len.to_usize() != self.bytes.len() {
            panic!("size not match.")
        }
        res.append(&mut self.len.serialize());
        res.append(&mut self.bytes.clone());
        res
    }

    fn size(&self) -> usize {
        self.len.size() + self.len.to_usize()
    }

}


impl Parse for $class {

    fn parse(&mut self, buf: &[u8], seek: usize) -> Ret<usize> {
        let (obj, sk) = <$lenty>::create(&buf[seek..]) ?;
        self.len = obj;
        let conlen = self.len.to_usize();
        let nsk = seek + sk;
        let bts = buf_clip_mvsk!(buf[nsk..], conlen);
        self.bytes = bts.to_vec();
        Ok(nsk + conlen)
    }

}

impl Field for $class {

    fn new() -> $class {
        let sz = <$lenty>::from_uint(0);
        $class{
            len: sz,
            bytes: Vec::new(),
        }
    }
    
    // must & create function
    fnFieldMustCreate!($class);
}


impl $class {

    pub fn from_vec_u8(v: Vec<u8>) -> $class {
        $class{
            len: <$lenty>::from_uint(v.len() as u64),
            bytes: v,
        }

    }

    pub fn length(&self) -> usize {
        self.len.to_usize()
    }

    pub fn push(&mut self, a: u8) -> Option<Error> {
        if self.bytes.len() + 1 > $size_max {
            return Some(s!("append size overflow"))
        }
        self.len += 1u8;
        self.bytes.push(a);
        None
    }

    pub fn concat(&mut self, tar: &[u8]) -> Option<Error> {
        let apsz = tar.len();
        if self.bytes.len() + apsz > $size_max {
            return Some(s!("concat size overflow"))
        }
        self.len += apsz as u64;
        self.bytes = [self.bytes.clone(), tar.to_vec()].concat();
        None
    }

}


    )
}

