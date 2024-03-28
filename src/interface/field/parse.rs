
pub trait Parse {
    // ret: next move seek or error
    fn parse(&mut self, buf: &[u8], seek: usize) -> Result<usize, Error>;
}


pub trait Cutout {
    // ret: body or error
    fn cutout<'a>(buf: &'a[u8], seek: usize) -> Result<&'a[u8], Error> where Self: Sized;
}

