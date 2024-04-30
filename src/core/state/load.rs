

pub fn load_block_package(store: &CoreStoreDisk, rhei: u64) -> Ret<Box<dyn BlockPkg>> {
    let hei = BlockHeight::from_uint(rhei);
    load_block_package_by_height(store, &hei)
}

pub fn load_block_package_by_height(store: &CoreStoreDisk, rhei: &BlockHeight) -> Ret<Box<dyn BlockPkg>> {

    // read block data
    let rblkbts = store.blockdatabyptr(rhei);
    if let None = rblkbts {
        let rhein = rhei.to_u64();
        return errf!("block store database error: not find block hash by height {}", rhein)
    }
    let rblkbts = rblkbts.unwrap();
    // println!("store.blockdata {}", hex::encode(rblkbts.as_ref()));
    // create pkg
    Ok(block::create_pkg(rblkbts).expect(&format!("load block {} store data error", rhei)))
}


