
pub struct ASTList {
    count: Uint2,
    lists: Vec<Box<dyn VMAction>>,
}

/* */
impl Parse for ASTList {
    /*fn parse(&mut self, buf: &[u8], seek: usize) -> Result<usize, Error> {
        let mut seek = self.count.parse(buf, seek) ? ;
        let num = self.count.to_usize();
        for i in 0..num {
            
        }
        Ok(seek)
    }*/
}

impl Serialize for ASTList {
    fn serialize(&self) -> Vec<u8> {
        let num = self.count.to_usize();
        let mut bts = vec![ self.count.serialize() ];
        for i in 0..num {
            bts.push( self.lists[i].serialize() );
        }
        bts.concat()
    }
    fn size(&self) -> usize {
        let num = self.count.to_usize();
        let mut size = self.count.size();
        for i in 0..num {
            size += self.lists[i].size();
        }
        size
    }
}

