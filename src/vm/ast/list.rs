
pub struct ASTList {
    count: Uint2,
    lists: Vec<Box<dyn VMAction>>,
}

impl Parse for ASTList {
    fn parse(&mut self, buf: &[u8], seek: usize) -> Result<usize, Error> {
        let mut seek = self.count.parse(buf, seek) ? ;
        let num = self.count.to_usize();
        for i in 0..num {
            
        }
        Ok(seek)
    }
}

