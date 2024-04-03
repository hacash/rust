

fn _locate_chunk(rtck: &Arc<RollChunk>, hx: &Hash) -> Option<Arc<RollChunk>> {
    let chk = rtck;
    if chk.hash == *hx {
        return Some(rtck.clone())
    }
    for a in chk.childs.borrow().iter() {
        let res = _locate_chunk(a, hx);
        if let Some(ck) = res {
            return Some(ck.clone())
        }
    }
    // not find
    None
}


// find
pub fn locate_base_chunk(this: &StateRoller, hx: &Hash) -> Option<Arc<RollChunk>> {
    _locate_chunk(&this.sroot, hx)
}
