
fn load_base_block(mintk: &dyn MintChecker, storef: &BlockStore) -> Box<dyn BlockPkg> {
    let store = CoreStoreRead::wrap(storef);
    let status = store.status();
    let rhei = &status.root_height;
    let rhein = rhei.to_u64();
    if 0 == rhein {
        // genesis block
        return mintk.genesis()
    }
    // read block data
    let rhx = store.blockptr(rhei);
    if let None = rhx {
        panic!("block store database error: not find block hash by height {}", rhein)
    }
    let rhx = rhx.unwrap();
    let rblkbts = store.blockdata(&rhx);
    if let None = rblkbts {
        panic!("block store database error: not find block data by hash {}", rhx)
    }
    let rblkbts = rblkbts.unwrap();
    // create pkg
    block::create_pkg(rblkbts)
}