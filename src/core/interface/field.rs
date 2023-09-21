

pub trait Serialize {
    fn new() -> Self where Self: Sized;
    fn serialize(&self) -> Vec<u8>;
    fn parse(&mut self, _: &Vec<u8>, _: usize) -> Result<usize, String>;
    fn size(&self) -> usize;
}

pub trait Describe {
    fn describe(&self) -> String; // readable format
    fn to_json(&self) -> String; // to json format string
    fn from_json(&mut self, _: &String) -> Result<(), String>; // from json
}

pub trait Number {
    fn to_u64(&self) -> u64;
    fn from_u64(&mut self, _: u64);
}

pub trait ToHex {
    fn to_hex(&self) -> String;
}

pub trait ToVecU8 {
    fn to_vec_u8(&self) -> Vec<u8>;
}


