
/**
 * do change chunk roller and state head
 */
fn do_roll(this: &mut BlockChainKernel, blkpkg: Box<dyn BlockPkg>, bsck: Arc<ChunkRoller>, state: Arc<ChainState>) -> Option<Error> {
    let istprevhx = *blkpkg.objc().prevhash();
    let mut chunk = ChunkRoller::create(blkpkg, state.clone());
    chunk.set_parent(Arc::downgrade(&bsck).into()); // set base chunk be parent
    let chunkobj = Arc::new(chunk);
    bsck.push_child(chunkobj.clone()); // push child
    // check move root
    let cshx = this.scusp.upgrade()?.hash;
    if istprevhx != cshx {
        // insert to fork so not move
        return None
    }
    let croothei = this.sroot.height.to_u64();
    let curckhei = this.scusp.upgrade()?.height.to_u64();
    // if croothei + this.cnf.unstable_block >= curckhei {
    //     // insert to fork so not move
    //     return None
    // }
    let mut newrootck = this.scusp.clone();
    for i in 0..this.cnf.unstable_block - 1 {
        if let Some(p) = newrootck.upgrade()?.parent.borrow().as_ref() {
            newrootck = p.clone();
        }else{
            break;
        }
    }
    let nh1 = newrootck.upgrade()?.height.to_u64();
    if croothei + 1 != nh1 {
        return erf!("insert to move current root height error, need {} but got {}",
            croothei + 1, nh1)
    }
    // roll chunk state
    let tarrt = newrootck.upgrade() ? ;
    do_roll_chunk_state(this, this.sroot.clone(), tarrt.clone()) ? ;
    // change state head and root
    this.scusp = Arc::downgrade(&chunkobj);
    this.state = Arc::downgrade(&state);
    this.sroot = tarrt;
    // ok
    None
}



/**
 * roll chunk state
 */
fn do_roll_chunk_state(this: &mut BlockChainKernel, base: Arc<ChunkRoller>, tar:Arc<ChunkRoller>) -> Option<Error> {

    

    None
}
