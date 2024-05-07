

#[derive(Default)]
pub struct ASTExtWrap {
    body: Vec<u8>,
}

/*
impl VMAction for ASTExtWrap {
    // fn body(&self) -> &[u8] { 
    //     &self.body
    // }
}
*/

impl Serialize for ASTExtWrap {
    fn serialize(&self) -> Vec<u8> {
        self.body.clone()
    }
    fn size(&self) -> usize {
        self.body.len()
    }
}


impl Parse for ASTExtWrap {
    
    fn parse(&mut self, buf: &[u8], seek: usize) -> Result<usize, Error> {
        Ok(seek)
    }
}


impl Field for ASTExtWrap {
    
}
