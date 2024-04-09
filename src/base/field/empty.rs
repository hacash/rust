
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Empty {
}


impl Serialize for Empty {

    fn serialize(&self) -> Vec<u8> {
        vec![]
    }

    fn size(&self) -> usize {
        0
    }

}


impl Parse for Empty {

    fn parse(&mut self, buf: &[u8], seek: usize) -> Result<usize, Error> {
        Ok(seek)    
    }

}

impl Field for Empty {

    fn new() -> Empty {
        Empty{}
    }

}
