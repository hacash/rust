use std::any::Any;


fn bbb(ext: &dyn Any) -> bool {

    if ext.is::<String>() {
        return true
    }

    false
}

fn build_ast_item(extcl: &dyn ExtActCaller, buf: &[u8]) -> Result<(Box<dyn VMAction>, usize), Error> {
    if buf.len() < 1 {
        return err_buf_short!()
    }

    // let extany = extcl as &dyn Any;
    // if extcl.is::<String>() {
    //     return err!("not")
    // }

    let opcd = buf[0];
    // if ext action
    if opcd == 0 {
        let body = extcl.cutout(buf) ? ;
        let size = body.len();
        return Ok((Box::new(ASTExtWrap{body}), size))
    }
    // if vm action
    if opcd == OPC_NOP {
        let size: usize = 1;
        return Ok((Box::new(ASTLeaf::from(opcd)), size))
    }

    // error
    err!( format!("cannot find opcode {}", opcd) )
}

