

/**
 * store block
 */
fn do_store(cnf: &EngineConf, storef: &BlockStore, roller: &mut BlockRoller, append: Arc<RollChunk>, change_status: RollerChangeStatus) -> RetErr {
    let rcs = change_status;
    let store = CoreStoreDisk::wrap(storef);
    // save block
    let blkhei = &append.height;
    let blkhx = append.block.hash();
    let blkbd = append.block.body();
    store.put_blockdata(blkhx, blkbd);
    // println!("put_blockdata {} {}", blkhei, blkhx);
    // if append
    if rcs.append {
        // save append ptr
        store.put_blockptr(blkhei, blkhx);
        // println!("put_blockptr {} {}", blkhei, blkhx);
    }
    // if roll
    if rcs.roll {
        let status = StoreStatus{
            root_height: roller.root_height(),
            last_height: roller.last_height(),
        };
        // println!("===> roll root status height {}", roller.sroot.height.to_u64());
        store.put_status(&status);
    }
    // if switch fork 
    if rcs.switchfork {
        let mut prev = append.clone();
        loop {
            if let Some(p) = prev.parent.upgrade() {
                // change ptr
                // println!("switch fork ^^^^ put_blockptr {} {}", &p.height, &p.hash);
                store.put_blockptr(&p.height, &p.hash);
                prev = p; // prev
            }else{
                break; // end
            }
        }
    }
    // finish
    Ok(())
}