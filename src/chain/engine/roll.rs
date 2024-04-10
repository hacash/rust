
/**
 * do change chunk roller and state head
 */
fn do_roll(cnf: &EngineConf, roller: &mut BlockRoller, append: Arc<RollChunk>) -> RetErr {
    let oldhead = roller.scusp.upgrade().unwrap();
    let root_height = roller.sroot.height.to_u64();
    let oldhead_height = oldhead.height.to_u64(); 
    let append_height = append.height.to_u64();
    if append_height <= oldhead_height {
        // not change head
        return Ok(())
    }
    // if roll
    if append_height <= root_height + cnf.unstable_block {
        // not roll
        return Ok(())
    }
    // do roll
    let mut new_root: Option<Arc<RollChunk>> = None;
    {
        let rtchilds = roller.sroot.childs.borrow();
        if rtchilds.len() == 1 {
            // only one child
            new_root = Some(rtchilds[0].clone());
        }
    }
    if let None = new_root {
        // search form new head to root
        let start = Arc::downgrade(&append);
        new_root = scan_parent_chunk(start, cnf.unstable_block);
    }
    if let None = new_root {
        return errf!("cannot find the base root chunk")
    }
    let new_root = new_root.unwrap();
    if new_root.height.to_u64() != root_height + 1 {
        return errf!("root chunk height error")
    }
    // change head
    roller.sroot.state.flush_disk(); // flush to disk
    new_root.drop_parent();
    roller.sroot = new_root;
    roller.scusp = Arc::downgrade(&append);
    roller.state = Arc::downgrade(&append.state);
    Ok(())
}


/**
 * scan parent chunk
 */
fn scan_parent_chunk(sub: Weak<RollChunk>, step: u64) -> Option<Arc<RollChunk>> {
    let up = sub.upgrade();
    if step == 0 {
        return up
    }
    // 
    return match up {
        Some(b) => scan_parent_chunk((*b.parent.borrow()).clone(), step-1),
        None => None,
    }
}















/*



fn do_roll_old(cnf: &EngineConf, this: &BlockRoller, blkpkg: Box<dyn BlockPkg>, bsck: Arc<RollChunk>, state: Arc<ChainState>) 
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

*/



/*
// roll chunk state
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

*/

