
pub trait Parse {
    // ret: next move seek or error
    fn parse(&mut self, buf: &[u8], seek: usize) -> Result<usize, Error> { panic_never_call_this!() }
}


pub trait Cutout {
    // ret: body or error
    fn cutout(buf: &[u8], seek: usize) -> Result<&[u8], Error> where Self: Sized;
}

