
pub fn create(buf: &[u8]) -> Ret<(Box<dyn Block>, usize)> {
    // println!("block::create {}", hex::encode(buf));
    let bts = buf_clip_mvsk!(buf, 1);
    let version = bts[0] as u8;
    match version {
        BLOCK_VERSION_1 => {
            let (blk, mvsk) = BlockV1::create(buf)?;
            Ok((Box::new(blk), mvsk))
        }
        _ => errf!("Block Type <{}> not find.", version)
    }
}


pub fn create_pkg(bytes: BytesW4) -> Ret<Box<dyn BlockPkg>> {
    let buf = bytes.as_ref();
    let (blkobj, _) = create(buf)?;
    let hash = blkobj.hash();
    Ok(Box::new(BlockPackage::new_with_data(blkobj, bytes.into_vec())))
}
