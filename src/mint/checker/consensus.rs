
fn impl_prepare(this: &BlockMintChecker, sto: &dyn Store, curblk: &dyn BlockRead) -> RetErr {
    let curhei = curblk.height().uint(); // u64
    if curhei < 288*200 {
        return Ok(()) // not check, compatible history code
    }
    let blkspan = this.cnf.difficulty_adjust_blocks;
    if curhei % blkspan == 0 {
        return Ok(()) // not check, difficulty update
    }
    // check
    let (_, diffhx) = this.difficulty.req_cycle_block(curhei, sto);
    let cblkhx = curblk.hash();
    if hash_big_than(cblkhx.as_ref(), &diffhx) {
        return errf!("block {} PoW difficulty check failed need less than {} but got {}", 
            curhei, hex::encode(diffhx),  hex::encode(cblkhx))
    }
    // check success
    Ok(())
}



fn impl_consensus(this: &BlockMintChecker, sto: &dyn Store, prevblk: &dyn BlockRead, curblk: &dyn BlockRead) -> RetErr {
    let curhei = curblk.height().uint(); // u64
    if curhei < 288*200 {
        return Ok(()) // not check, compatible history code
    }
    // check
    let curn = curblk.difficulty().uint(); // u32
    let curbign = u32_to_biguint(curn);
    let prevn = prevblk.difficulty().uint(); // u32
    let prevtime = prevblk.timestamp().uint(); // u64
    let (tarn, tarhx, tarbign) = this.difficulty.target(prevn, prevtime, curhei, sto);
    // check
    /*if curbign!=tarbign || tarn!=curn || tarhx!=u32_to_hash(curn) {
        println!("\nheight: {}, {} {}, tarhx: {}  curhx: {} ----------------", 
        curhei, tarn, curn, hex::encode(&tarhx), hex::encode(u32_to_hash(curn)));
        return errf!("curbign != tarbign")
    }*/
    if curbign > tarbign {
        return errf!("block {} PoW difficulty check failed need less than {} but got {}", curhei, tarn, curn)
    }
    // success
    Ok(())
}