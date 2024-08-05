

// create Bytes macro
macro_rules! StructFieldBytes{
    ($class:ident, $lenty:ty, $size_max:expr) => (

#[derive(Default, Debug, Clone, Eq)]
pub struct $class {
    pub count: $lenty,
    pub bytes: Vec<u8>,
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
        if self.count.to_usize() != self.bytes.len() {
            panic!("size not match.")
        }
        res.append(&mut self.count.serialize());
        res.append(&mut self.bytes.clone());
        res
    }

    fn size(&self) -> usize {
        self.count.size() + self.count.to_usize()
    }

}


impl Parse for $class {

    fn parse(&mut self, buf: &[u8], seek: usize) -> Ret<usize> {
        let (obj, sk) = <$lenty>::create(&buf[seek..]) ?;
        self.count = obj;
        let conlen = self.count.to_usize();
        let nsk = seek + sk;
        let bts = buf_clip_mvsk!(buf[nsk..], conlen);
        self.bytes = bts.to_vec();
        Ok(nsk + conlen)
    }

}

impl Field for $class {
    
    // must & create function
    fnFieldMustCreate!($class);
}


impl $class {

    pub fn from_vec(v: Vec<u8>) -> $class {
        $class{
            count: <$lenty>::from_uint(v.len() as u64),
            bytes: v,
        }

    }

    pub fn into_vec(mut self) -> Vec<u8> {
        self.bytes
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn length(&self) -> usize {
        self.count.to_usize()
    }

    pub fn push(&mut self, a: u8) -> RetErr {
        if self.bytes.len() + 1 > $size_max {
            return errf!("append size overflow")
        }
        self.count += 1u8;
        self.bytes.push(a);
        Ok(())
    }

    pub fn append(&mut self, tar: &mut Vec<u8>) -> RetErr {
        self.count += tar.len() as u64;
        self.bytes.append(tar);
        Ok(())
    }

}


    )
}

