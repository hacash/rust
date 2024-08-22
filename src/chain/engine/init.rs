
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
    let status = store.status();
    // next
    let mut next_height: u64 = {
        let chei = this.klctx.lock().unwrap().sroot.height.to_u64();
        chei
    };
    // build unstable blocks 
    let finish_height = status.last_height.uint();
    let is_all_rebuild = finish_height - next_height > 10;
    if is_all_rebuild {
        println!("[Database] rebuild all blocks to upgrade data version, plase waiting...");
    }else{
        print!("[Engine] Data: {}, rebuild ({})", this.cnf.data_dir, next_height);
    }
    // insert lock
    this.isrlck.lock();
    loop {
        next_height += 1;
        let resblk = load_block_package(&store, next_height);
        if let Err(..) = resblk {
            println!(" ok.");
            return // end finish
        }
        let resblk = resblk.unwrap();
        if is_all_rebuild {
            if next_height % 500 == 0 {
                let per = next_height as f32 / finish_height as f32;
                print!("\r{:10} ({:.2}%)", next_height, per*100.0);
            }
        } else {
            print!("➢{}", next_height);
        }
        // try insert
        let ier = this.insert_unsafe(resblk);
        if let Err(e) = ier {
            panic!("[State Panic] rebuild block state error: {}", e);
        }
        // next
        std::io::stdout().flush().unwrap();
    }
}

