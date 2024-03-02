

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

    fn parse(&mut self, buf: &[u8], seek: usize) -> Result<usize, Error> {
        let (obj, sk) = <$lenty>::create(buf[seek..]) ?;
        self.len = obj;
        let conlen = self.len.to_usize();
        let bts = buf_clip_mvsk!(buf, strlen);
        self.bytes = bts.to_vec();
        Ok(sk + conlen)
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

    fn from(buf: &[u8]) -> Self where Self: Sized { 
        if buf.len() > $size_max {
            panic!("size overflow max {}", $size_max)
        }
        let v = buf.clone();
        // obj
        let mut obj = <$class>::new();
        obj.len = <$lenty>::from_uint(v.len() as u64);
        obj.bytes = v.try_into().unwrap();
        // ok
        obj
    }

    // create function
    fnFieldCreate!($class);
}


impl $class {



}


    )
}

