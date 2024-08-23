
/**
 * do change chunk roller and state head
 * return (Append, Roll, Switch)
 */
fn do_roll(eng: &BlockEngine, cnf: &EngineConf, roller: &mut BlockRoller, append: Arc<RollChunk>) -> Ret<RollerChangeStatus> {
    let mut rcs_status = RollerChangeStatus::new();
    
    let oldhead = roller.scusp.upgrade().unwrap();
    let root_height = roller.sroot.height.to_u64();
    let oldhead_height = oldhead.height.to_u64(); 
    let append_height = append.height.to_u64();
    if append_height <= oldhead_height {
        // not change head
        return Ok(rcs_status) // just uncle
    }
    // change head
    roller.scusp = Arc::downgrade(&append);
    roller.state = Arc::downgrade(&append.state);
    rcs_status.append = true; // append
    if oldhead.hash != *append.block.objc().prevhash() {
        rcs_status.switchfork = true; // switch fork !!!
    }
    // if roll
    if append_height <= root_height + cnf.unstable_block {
        // not roll
        return Ok(rcs_status)
    }
    rcs_status.roll = true; // roll
    // do roll to disk
    let mut new_root: Option<Arc<RollChunk>> = None;
    {
        let rtchilds = roller.sroot.childs.lock().unwrap();
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
    // extend roll notice
    let scres = eng.blkscaner.roll(new_root.block.clone(), new_root.state.clone(), eng.store.clone());
    if let Err(e) = scres {
        panic!("\n\nBlock scaner error: {}\n\n", e);
    }
    // change root
    roller.sroot = new_root.clone(); // roll
    // roll root 
    // println!("!!!!!! flush_disk height {}", new_root.height.to_u64());
    new_root.state.flush_disk(); // flush to disk
    // new_root.drop_parent(); // do roll auto drop parent
    // ok
    return Ok(rcs_status)
    
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
        Some(b) => scan_parent_chunk(b.parent.clone(), step-1),
        None => None,
    }
}











