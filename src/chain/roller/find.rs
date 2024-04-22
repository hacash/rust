

fn _search_chunk(rtck: &Arc<RollChunk>, hx: &Hash) -> Option<Arc<RollChunk>> {
    let chk = rtck;
    if chk.hash == *hx {
        return Some(rtck.clone())
    }
    for a in chk.childs.lock().unwrap().iter() {
        let res = _search_chunk(a, hx);
        if let Some(ck) = res {
            return Some(ck.clone())
        }
    }
    // not find
    None
}


// find
pub fn locate_base_chunk(roller: &BlockRoller, hx: &Hash) -> Option<Arc<RollChunk>> {
    if let Some(b) = roller.scusp.upgrade() {
        if b.hash == *hx {
            return Some(b) // is leaf
        }
    }
    // search form root
    _search_chunk(&roller.sroot, hx)
}
