
pub struct ASTExtWrap {
    body: Vec<u8>,
}


impl VMAction for ASTExtWrap {
    fn ext_body(&self) -> &[u8] { 
        &self.body
    }
}

impl Serialize for ASTExtWrap {
    fn serialize(&self) -> Vec<u8> {
        self.body.clone()
    }
    fn size(&self) -> usize {
        self.body.len()
    }
}


impl Parse for ASTExtWrap {
    
}


impl Field for ASTExtWrap {
    fn new() -> ASTExtWrap {
        ASTExtWrap{
            body: vec![],
        }
    }
}
