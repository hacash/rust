
/**
 * do change chunk roller and state head
 */
fn do_roll(cnf: &KernelConf, this: &mut KernelCtx, blkpkg: Box<dyn BlockPkg>, bsck: Arc<ChunkRoller>, state: Arc<ChainState>) -> RetErr {
    let istprevhx = *blkpkg.objc().prevhash();
    let mut chunk = ChunkRoller::create(blkpkg, state.clone());
    chunk.set_parent(Arc::downgrade(&bsck).into()); // set base chunk be parent
    let chunkobj = Arc::new(chunk);
    bsck.push_child(chunkobj.clone()); // push child
    // check move root
    let cshx = this.scusp.upgrade().unwrap().hash;
    if istprevhx != cshx {
        // insert to fork so not move
        return Ok(())
    }
    let croothei = this.sroot.height.to_u64();
    let curckhei = this.scusp.upgrade().unwrap().height.to_u64();
    // if croothei + this.cnf.unstable_block >= curckhei {
    //     // insert to fork so not move
    //     return None
    // }
    let mut newrootck = this.scusp.clone();
    for i in 0..cnf.unstable_block - 1 {
        if let Some(p) = newrootck.upgrade().unwrap().parent.borrow().as_ref() {
            newrootck = p.clone();
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
    do_roll_chunk_state(this, this.sroot.clone(), tarrt.clone()) ? ;
    // change state head and root
    this.scusp = Arc::downgrade(&chunkobj);
    this.state = Arc::downgrade(&state);
    this.sroot = tarrt;
    // ok
    Ok(())
}



/**
 * roll chunk state
 */
fn do_roll_chunk_state(this: &mut KernelCtx, base: Arc<ChunkRoller>, tar:Arc<ChunkRoller>) -> RetErr {

    

    Ok(())
}
