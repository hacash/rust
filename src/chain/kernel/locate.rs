

fn _locate_chunk(rtck: &Arc<ChunkRoller>, hx: &Hash) -> Option<Arc<ChunkRoller>> {
    if rtck.hash == *hx {
        return Some(rtck.clone())
    }
    for a in rtck.childs.iter() {
        let res = _locate_chunk(a, hx);
        if let Some(ck) = res {
            return Some(ck.clone())
        }
    }
    // not find
    None
}


// find
fn locate_base_chunk(this: &BlockChainKernel, hx: &Hash) -> Option<Arc<ChunkRoller>> {
    _locate_chunk(&this.sroot, hx)
}
