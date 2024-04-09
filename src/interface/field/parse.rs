
pub trait Parse {
    // ret: next move seek or error
    fn parse(&mut self, buf: &[u8], seek: usize) -> Ret<usize>;
}


pub trait Cutout {
    // ret: body or error
    fn cutout<'a>(buf: &'a[u8], seek: usize) -> Ret<&'a[u8]> where Self: Sized;
}

