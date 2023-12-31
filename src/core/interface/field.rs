use super::super::Error;
use crate::core::base::*;

pub trait Serialize {
    fn parse(&mut self, _: &Vec<u8>, _: usize) -> Result<usize, Error>;
    fn serialize(&self) -> Vec<u8>;
    fn size(&self) -> usize;
}

pub trait Describe {
    fn describe(&self) -> String { "*".to_string() } // readable format
    fn to_json(&self, _: &FieldJsonConfig) -> String { "*".to_string() } // to json format string
    fn from_json(&mut self, _: &String) -> Option<Error> { panic_never_call_this!() } // from json
}

pub trait Field : Serialize + Describe {
    fn new() -> Self where Self: Sized;
    // fn length() -> usize { panic_never_call_this!() }
    fn create(_: &Vec<u8>, _: usize) -> Result<(Self, usize), Error> where Self: Sized { panic_never_call_this!() }
    fn from_uint<T>(_: T) -> Self where Self: Sized, T: std::ops::Add<u64, Output = u64> { panic_never_call_this!() }
    fn from_float<T>(nt: T) -> Self where Self: Sized, T: std::ops::Add<f64, Output = f64> { panic_never_call_this!() }
    fn from_u8s(_: impl AsRef<[u8]>) -> Self where Self: Sized { panic_never_call_this!() }
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
    fn to_u8(&self) -> u8 {0}
    fn from_u8(&mut self, _: u8) { panic_never_call_this!() }
    fn to_u16(&self) -> u16 {0}
    fn from_u16(&mut self, _: u16) { panic_never_call_this!() }
    fn to_u32(&self) -> u32 {0}
    fn from_u32(&mut self, _: u32) { panic_never_call_this!() }
    fn to_u64(&self) -> u64 {0}
    fn from_u64(&mut self, _: u64) { panic_never_call_this!() }

    fn to_f32(&self) -> f32 {0.0}
    fn from_f32(&mut self, _: f32) { panic_never_call_this!() }
    fn to_f64(&self) -> f64 {0.0}
    fn from_f64(&mut self, _: f64) { panic_never_call_this!() }
}

pub trait FieldReadableString : Field {
    fn to_readable_string(&self) -> String;
    fn from_readable_string(&mut self, _: &String) -> Option<Error>;
}

