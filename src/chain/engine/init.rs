
fn load_base_block(mintk: &dyn MintChecker, storef: &BlockStore) -> Box<dyn BlockPkg> {
    let store = CoreStoreDisk::wrap(storef);
    let status = store.status();
    let rhei = &status.root_height;
    let rhein = rhei.to_u64();
    if 0 == rhein {
        // genesis block
        return mintk.genesis_block().into()
    }
    // read block data
    let resblk = load_block_package_by_height(&store, &rhei);
    if let Err(e) = resblk {
        panic!("{}", e)
    }
    resblk.unwrap()
}


/******** rebuild ********/


impl BlockEngine {

    fn rebuild_unstable_blocks(&mut self) {
        _do_rebuild(self)
    }

}


fn _do_rebuild(this: &mut BlockEngine) {
    let store = CoreStoreDisk::wrap(this.store.as_ref());
    // next
    let mut next_height: u64 = {
        let chei = this.klctx.lock().unwrap().sroot.height.to_u64();
        chei
    };
    // build
    print!("[Engine] Datadir: {}, \n         Rebuild unstable blocks ({})", this.cnf.data_dir, next_height);
    // insert lock
    this.isrlck.lock();
    loop {
        next_height += 1;
        let resblk = load_block_package(&store, next_height);
        if let Err(_) = resblk {
            println!(" ok.");
            return // end finish
        }
        let blk = resblk.unwrap();
        print!(" -> {}", blk.objc().height().to_u64());
        // try insert
        let ier = this.insert_unsafe(blk); // ignore err
        if let Err(e) = ier {
            print!("[Error: {}]", e);
        }
        // next
    }
}

