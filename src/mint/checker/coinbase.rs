
fn impl_coinbase(this: &BlockMintChecker, height: u64, cbtx: &dyn Transaction) -> RetErr {
    let cbrw = cbtx.reward();
    let cbneed = block_reward(height);
    if *cbrw != cbneed {
        return errf!("block coinbase reward need {} but got {}", cbneed.to_fin_string(), cbrw.to_fin_string())
    }
    // ok
    Ok(())
}

