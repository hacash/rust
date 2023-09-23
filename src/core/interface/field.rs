use super::Error;

pub trait Serialize {
    fn parse(&mut self, _: &Vec<u8>, _: usize) -> Result<usize, Error>;
    fn serialize(&self) -> Vec<u8>;
    fn size(&self) -> usize;
}

pub trait Describe {
    fn describe(&self) -> String; // readable format
    fn to_json(&self) -> String; // to json format string
    fn from_json(&mut self, _: &String) -> Option<Error>; // from json
}

pub trait Field : Serialize + Describe {
    fn new() -> Self where Self: Sized;
}

pub trait FieldHex : Field {
    fn to_hex(&self) -> String;
    fn from_hex(&mut self, _: &String) -> Option<Error>;
}

pub trait FieldBytes : Field {
    fn to_vec_u8(&self) -> Vec<u8>;
    fn from_vec_u8(&mut self, _: &Vec<u8>) -> Option<Error>;
}

pub trait FieldNumber : Field {
    fn to_u64(&self) -> u64;
    fn from_u64(&mut self, _: &u64);
}

pub trait FieldReadableString : Field {
    fn to_string(&self) -> String;
    fn from_string(&mut self, _: &String) -> Option<Error>;
}

