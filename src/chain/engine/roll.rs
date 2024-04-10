
/**
 * do change chunk roller and state head
 */
fn do_roll(cnf: &EngineConf, this: &BlockRoller, blkpkg: Box<dyn BlockPkg>, bsck: Arc<RollChunk>, state: Arc<ChainState>) 
    -> Ret<Option<(Weak<RollChunk>, Weak<ChainState>, Arc<RollChunk>)>> {
    let istprevhx = *blkpkg.objc().prevhash();
    let mut chunk = RollChunk::create(blkpkg, state.clone());
    chunk.set_parent(bsck.clone()); // set base chunk be parent
    let chunkobj = Arc::new(chunk);
    bsck.push_child(chunkobj.clone()); // push child
    // check move root
    let cshx = this.scusp.upgrade().unwrap().hash;
    if istprevhx != cshx {
        // insert to fork so not move
        return Ok(None)
    }
    let croothei = this.sroot.height.to_u64();
    let curckhei = this.scusp.upgrade().unwrap().height.to_u64();
    // if croothei + this.cnf.unstable_block >= curckhei {
    //     // insert to fork so not move
    //     return None
    // }
    let mut newrootck = this.scusp.clone();
    for i in 0..cnf.unstable_block - 1 {
        if let Some(p) = (*newrootck.upgrade().unwrap().parent.borrow()).upgrade() {
            newrootck = Arc::downgrade(&p);
        }else{
            break;
        }
    }
    let nh1 = newrootck.upgrade().unwrap().height.to_u64();
    if croothei + 1 != nh1 {
        return errf!("insert to move current root height error, need {} but got {}",
            croothei + 1, nh1)
    }
    // roll chunk state
    let tarrt = newrootck.upgrade().unwrap();
    // return 
    // ok
    Ok(Some((Arc::downgrade(&chunkobj), Arc::downgrade(&state), tarrt)))
}



/**
 * roll chunk state
 */
fn do_roll_chunk_state(this: &mut BlockRoller, scusp: Weak<RollChunk>, state: Weak<ChainState>, sroot: Arc<RollChunk>) -> RetErr {

    // flush
    // sroot.state.flush_disk();

    // unset parent and drop
    sroot.drop_parent();

    this.scusp = scusp;
    this.state = state;
    this.sroot = sroot;

    Ok(())
}
