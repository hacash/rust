

// create Bytes macro
macro_rules! create_bytes_struct_and_impl{
    ($tip:expr, $class:ident, $lenty:ty, $size_max:expr) => (

#[derive(Clone)]
pub struct $class {
    len: $lenty,
    bytes: Vec<u8>,
}


impl Serialize for $class {

    fn serialize(&self) -> Vec<u8> {
        let lv = self.size();
        let mut res = Vec::with_capacity(lv);
        if self.len.to_u64() as usize != self.bytes.len() {
            panic!("{} size not match.", $tip)
        }
        res.append(&mut self.len.serialize());
        res.append(&mut self.bytes.clone());
        res
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, Error> {
        let (obj, seek) = parse_field_or_return_err!($tip, $lenty, buf, seek);
        self.len = obj;
        let strlen = self.len.to_u64() as usize;
        let seek2 = parse_move_seek_or_buf_too_short_error!($tip, seek, strlen, buf);
        let sv = &buf[seek..seek2];
        self.bytes = sv.to_vec();
        Ok(seek2)
    }

    fn size(&self) -> usize {
        self.len.size() + self.len.to_u64() as usize
    }

}


impl Describe for $class {

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

impl Field for $class {

    // create function
    pub_fn_field_create_by_new_wrap_return!($class);

    fn new() -> $class {
        let sz = <$lenty>::from_uint(0);
        $class{
            len: sz,
            bytes: Vec::new(),
        }
    }

    fn from(buf: impl AsRef<[u8]>) -> Self where Self: Sized { 
        let v = buf.as_ref().clone();
        if v.len() > $size_max {
            panic!("size overflow max {}", $size_max)
        }
        // obj
        let mut obj = <$class>::new();
        obj.len = <$lenty>::from_uint(v.len() as u64);
        obj.bytes = v.try_into().unwrap();
        // ok
        obj
    }

}


impl $class {



}


    )
}

