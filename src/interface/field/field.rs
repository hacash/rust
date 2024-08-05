
pub trait Field : Serialize + Parse { // Default
    fn new() -> Self where Self: Sized { panic_never_call_this!() }
    fn must(_: &[u8]) -> Self where Self: Sized { panic_never_call_this!(); } // maybe panic!
    fn build(_: &[u8]) -> Ret<Self> where Self: Sized { panic_never_call_this!(); }
    fn create(_: &[u8]) -> Ret<(Self, usize)> where Self: Sized { panic_never_call_this!(); }
    
    fn from_uint<T>(_: T) -> Self where Self: Sized, T: std::ops::Add<u64, Output = u64> { panic_never_call_this!(); }
    fn from_float<T>(_: T) -> Self where Self: Sized, T: std::ops::Add<f64, Output = f64> { panic_never_call_this!(); }
    // fn parse_uint<T>(&mut self, _: T) -> Option<Error> where T: std::ops::Add<u64, Output = u64> { panic_never_call_this!(); }
    // fn parse_float<T>(&mut self, _: T) -> Option<Error> where T: std::ops::Add<f64, Output = f64> { panic_never_call_this!(); }
}

pub trait FieldHex : Field {
    fn hex(&self) -> String { panic_never_call_this!(); }
    fn from_hex(_: &[u8]) -> Self where Self: Sized { panic_never_call_this!(); } // maybe panic!
    fn create_by_hex(_: &[u8]) -> Ret<(Self, usize)> where Self: Sized { panic_never_call_this!(); }
}

pub trait FieldUint : Field {
    fn to_u8(&self) -> u8 { panic_never_call_this!(); }
    fn to_u16(&self) -> u16 { panic_never_call_this!(); }
    fn to_u32(&self) -> u32 { panic_never_call_this!(); }
    fn to_u64(&self) -> u64 { panic_never_call_this!(); }
    fn to_usize(&self) -> usize { panic_never_call_this!(); }
    fn from_u8(_: u8) -> Self where Self: Sized { panic_never_call_this!(); } // panic
    fn from_u16(_: u16) -> Self where Self: Sized { panic_never_call_this!(); } // panic
    fn from_u32(_: u32) -> Self where Self: Sized { panic_never_call_this!(); } // panic
    fn from_u64(_: u64) -> Self where Self: Sized { panic_never_call_this!(); } // panic
    fn from_usize(_: usize) -> Self where Self: Sized { panic_never_call_this!(); } // panic
    fn parse_u8(&mut self, _: u8) -> RetErr { panic_never_call_this!(); } // panic
    fn parse_u16(&mut self, _: u16) -> RetErr { panic_never_call_this!(); } // panic
    fn parse_u32(&mut self, _: u32) -> RetErr { panic_never_call_this!(); } // panic
    fn parse_u64(&mut self, _: u64) -> RetErr { panic_never_call_this!(); } // panic
    fn parse_usize(&mut self, _: usize) -> RetErr { panic_never_call_this!(); } // panic

}

pub trait FieldFloat : Field {
    fn to_f32(&self) -> f32 { panic_never_call_this!(); }
    fn to_f64(&self) -> f64 { panic_never_call_this!(); }
    fn from_f32(_: f32) -> Self where Self: Sized { panic_never_call_this!(); } // panic
    fn from_f64(_: f64) -> Self where Self: Sized { panic_never_call_this!(); } // panic
    fn parse_f32(&mut self, _: f32) -> RetErr { panic_never_call_this!(); } // panic
    fn parse_f64(&mut self, _: f64) -> RetErr { panic_never_call_this!(); } // panic
}

pub trait FieldReadable : Field {
    fn readable(&self) -> String;
    fn from_readable(_: &[u8]) -> Self where Self: Sized { panic_never_call_this!(); } // panic
    fn create_by_readable(_: &[u8]) -> Ret<(Self, usize)> where Self: Sized { panic_never_call_this!(); }
}


