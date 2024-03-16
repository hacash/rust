


fn build_ast_item(extcl: &dyn ExtActCaller, buf: &[u8]) -> Result<(Box<dyn VMAction>, usize), Error> {
    if buf.len() < 1 {
        return err_buf_short!()
    }
    let bcode = buf[0];
    // if ext action
    if bcode == 0 {
        let body = extcl.cutout(buf) ? ;
        let size = body.len();
        return Ok((Box::new(ASTExtWrap{body}), size))
    }
    // if vm action
    

    

    err!("")
    
}

