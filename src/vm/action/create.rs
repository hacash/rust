

/**
 * create vm action
 */
pub fn create(buf: &[u8]) -> Ret<(Box<dyn VMAction>, usize)> {
    let cds = buf_clip_mvsk!(buf, 1);
    let code = cds[0] as u8;
    if code == 0xFE {
        // Abort Error
    }
    // try extend actions
    let kid = protocol::action::cut_kind(buf) ? ;
    // core
    let extactobj1 = protocol::action::try_create_vm(kid, buf) ? ;
    if let Some(res) = extactobj1 {
        return Ok(res)
    }
    // mint
    let mut extactobj2 = mint::action::try_create_vm(kid, buf) ? ;
    if let Some(res) = extactobj2 {
        return Ok(res)
    }
    // not find
    errf!("cannot find any action by code {} or kind {}", code, kid)
}



