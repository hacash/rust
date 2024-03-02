
pub trait Parse {
    // ret: next move seek or error
    fn parse(&mut self, buf: &[u8], seek: usize) -> Result<usize, Error>;
}

