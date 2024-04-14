

pub fn load_block_package(store: &CoreStoreDisk, rhei: u64) -> Ret<Box<dyn BlockPkg>> {
    let hei = BlockHeight::from_uint(rhei);
    load_block_package_by_height(store, &hei)
}

pub fn load_block_package_by_height(store: &CoreStoreDisk, rhei: &BlockHeight) -> Ret<Box<dyn BlockPkg>> {

    // read block data
    let rhx = store.blockptr(rhei);
    if let None = rhx {
        let rhein = rhei.to_u64();
        return errf!("block store database error: not find block hash by height {}", rhein)
    }
    let rhx = rhx.unwrap();
    let rblkbts = store.blockdata(&rhx);
    if let None = rblkbts {
        return errf!("block store database error: not find block data by hash {}", rhx)
    }
    let rblkbts = rblkbts.unwrap();
    // println!("store.blockdata {}", hex::encode(rblkbts.as_ref()));
    // create pkg
    Ok(block::create_pkg(rblkbts).expect(&format!("load block {} store data error", rhei)))
}


