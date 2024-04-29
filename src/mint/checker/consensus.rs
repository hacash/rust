
fn impl_consensus(this: &BlockMintChecker, sto: &dyn Store, prevblk: &dyn Block, curblk: &dyn Block) -> RetErr {
    let curhei = curblk.height().uint(); // u64
    let curn = curblk.difficulty().uint(); // u32
    let curbign = u32_to_biguint(curn);
    let prevn = prevblk.difficulty().uint(); // u32
    let prevtime = prevblk.timestamp().uint(); // u64
    let (tarn, tarhx, tarbign) = this.difficulty.target(prevn, prevtime, curhei, sto);
    // check

    if curhei < 288*450 {
        return Ok(())
    }

    if curbign!=tarbign || tarn!=curn || tarhx!=u32_to_hash(curn) {

        println!("\nheight: {}, {} {}, tarhx: {}  curhx: {} ----------------", 
        curhei, tarn, curn, hex::encode(&tarhx), hex::encode(u32_to_hash(curn)));
        return errf!("curbign != tarbign")
    }

    if curbign > tarbign {
        return errf!("block PoW difficulty check failed need less than {} but got {}", tarn, curn)
    }
    // success
    Ok(())
}