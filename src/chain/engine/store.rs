

/**
 * store block
 */
fn do_store(cnf: &EngineConf, storef: &BlockStore, roller: &mut BlockRoller, append: Arc<RollChunk>, change_status: RollerChangeStatus) -> RetErr {
    let store = CoreStoreRead::wrap(storef.as_db());
    // save block
    let blkhei = &append.height;
    let blkhx = append.block.hash();
    let blkbd = append.block.body();
    store.set_blockdata(blkhx, blkbd);
    if let RollerChangeStatus::Uncle = change_status {
        return Ok(())
    }
    // save append ptr
    store.set_blockptr(blkhei, blkhx);
    if let RollerChangeStatus::Append = change_status {
        return Ok(()) // not roll
    }
    // save roll status
    let status = StoreStatus{
        root_height: roller.sroot.height.clone(),
    };
    store.set_status(&status);
    if let RollerChangeStatus::AppendRoll = change_status {
        return Ok(()) // not switch fork
    }
    // switch fork // AppendRollSwitchFork
    let mut prev = append.clone();
    for i in 0 .. cnf.unstable_block {
        if let Some(p) = prev.parent.upgrade() {
            // change ptr
            store.set_blockptr(&p.height, &p.hash);
            prev = p; // prev
        }else{
            break; // end
        }
    }
    // ok
    Ok(())
}