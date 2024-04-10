
/**
 * do insert block crate new state
 * return new chunk and state
 */
pub fn do_check_insert(

    cnf: &EngineConf, 
    mintk: &dyn MintChecker, 
    prev_state: Arc<ChainState>, 
    prev_block: &dyn Block, 
    blkpkg: &dyn BlockPkg

) -> Ret<ChainState> {

    // check height
    let block = blkpkg.objc();
    // let prev_block = prev_block;
    let height = block.height().to_u64();
    let prev_height = prev_block.height().to_u64();
    if height != prev_height + 1 {
        return errf!("block height {} is not match to insert, need {}", height, prev_height + 1)
    }
    // check prev hash
    let prev_hx = block.prevhash();
    let base_hx = prev_block.hash();
    if *prev_hx != base_hx {
        return errf!("need prev hash {} but got {}", base_hx, prev_hx)
    };
    // check time
    let prev_blk_time = prev_block.timestamp().to_u64();
    let blk_time = block.timestamp().to_u64();
    let cur_time = curtimes();
    if blk_time > cur_time {
        return errf!("block timestamp {} cannot more than system timestamp {}", blk_time, cur_time)
    }
    if blk_time <= prev_blk_time {
        return errf!("block timestamp {} cannot less than prev block timestamp {}", blk_time, prev_blk_time)
    }
    // check size
    let blk_size = blkpkg.body().len();
    if blk_size > cnf.max_block_size + 100 { // may 1MB + headmeta size
        return errf!("block size cannot over {} bytes", cnf.max_block_size + 100)
    }
    // check tx count
    let is_hash_with_fee = true;
    let txhxs = block.transaction_hash_list(is_hash_with_fee); // hash with fee
    let txcount = block.transaction_count() as usize;
    if txcount < 1 {
        return err!("block txs cannot empty, need coinbase tx")
    }
    if txcount > cnf.max_block_txs { // may 999
        return errf!("block txs cannot more than {}", cnf.max_block_txs)
    }
    if txcount != txhxs.len() {
        return errf!("block tx count need {} but got {}", txhxs.len(), txcount)
    }
    // check tx total size and count
    let alltxs = block.transactions();
    let mut txttsize = 0usize;
    let mut txttnum = 0usize;
    for tx in alltxs.iter() {
        txttsize += tx.size();
        txttnum += 1;
        if tx.timestamp().to_u64() > cur_time {
            return errf!("tx timestamp {} cannot more than now {}", tx.timestamp(), cur_time)
        }
    }
    if txttnum != txcount {
        return errf!("block tx count need {} but got {}", txcount, txttnum)        
    }
    if txttsize > cnf.max_block_size { // may 1MB
        return errf!("block txs total size cannot over {} bytes", cnf.max_block_size)
    }
    // check mrkl root
    let mkroot = merge_mrkl_root(&txhxs);
    let mrklrt = block.mrklroot();
    if *mrklrt != mkroot {
        return errf!("block mrkl root need {} but got {}", mkroot, mrklrt)
    }
    // check mint consensus & coinbase
    mintk.consensus(&**block) ? ;
    // coinbase tx id = 0, if coinbase error
    mintk.coinbase(height, &*alltxs[0]) ? ;
    // check state
    let mut sub_state = fork_sub_state(prev_state.clone());
    // if init genesis status
    if height == 1 {
        // state initialize 
        mintk.initialize(&mut sub_state) ? ;
    }
    // exec each tx
    for tx in alltxs.iter() {
        // let mut txstate = fork_sub_state(txstabs.clone());
        // kernel.vmobj.exec_tx( tx.to_readonly(), &mut sub_state) ? ;
        // ok merge copy state
        // sub_state.merge_copy(substate.as_ref()) ? ;
    }
    // 
    


    // test
    Ok(sub_state)

}




pub fn do_insert(kernel: &BlockEngine, cnf: &EngineConf, this: &BlockRoller, mintk: &dyn MintChecker, blkpkg: &dyn BlockPkg) -> Ret<(Arc<RollChunk>, Arc<ChainState>)> {

    // check height
    let block = blkpkg.objc();
    let isrhei = block.height().to_u64();
    let rthei = this.sroot.height.to_u64();
    if isrhei <= rthei {
        return errf!("block height {} is too low to insert, need above {}", isrhei, rthei)
    }
    let underhei = rthei + cnf.unstable_block + 1; // unstable block = 4
    if isrhei > underhei {
        return errf!("block height {} is too high to insert, need equal or under {}", isrhei, underhei)
    }
    // check prev hash
    let prevhx = block.prevhash();
    let prevchunk = roller::locate_base_chunk(this, prevhx);
    let Some(prevchunk) = prevchunk else {
        return errf!("cannot find base block by prev hash {}", prevhx)
    };
    let prevhei = prevchunk.height.to_u64();
    if prevhei+1 != isrhei {
        return errf!("block height need {} but got {}", prevhei+1, isrhei)
    }
    // check time
    let prevblkts = prevchunk.block.objc().timestamp().to_u64();
    let blkts = block.timestamp().to_u64();
    let ctis = curtimes();
    if blkts > ctis {
        return errf!("block timestamp {} cannot more than system timestamp {}", blkts, ctis)
    }
    if blkts <= prevblkts {
        return errf!("block timestamp {} cannot less than prev block {}", blkts, prevblkts)
    }
    // check size
    let blksz = blkpkg.body().len();
    if blksz > cnf.max_block_size + 90 { // may 1MB + headmeta size
        return errf!("block size cannot over {} bytes", cnf.max_block_size + 90)
    }
    // check tx count
    let is_hash_with_fee = true;
    let txhxs = block.transaction_hash_list(is_hash_with_fee); // hash with fee
    let txcount = block.transaction_count() as usize;
    if txcount < 1 {
        return err!("block txs cannot empty, need coinbase tx")
    }
    if txcount > cnf.max_block_txs { // may 999
        return errf!("block txs cannot more than {}", cnf.max_block_txs)
    }
    if txcount != txhxs.len() {
        return errf!("block tx count need {} but got {}", txhxs.len(), txcount)
    }
    // check tx total size and count
    let alltxs = block.transactions();
    let mut txttsize = 0usize;
    let mut txttnum = 0usize;
    for tx in alltxs.iter() {
        txttsize += tx.size();
        txttnum += 1;
        if tx.timestamp().to_u64() > ctis {
            return errf!("tx timestamp {} cannot more than now {}", tx.timestamp(), ctis)
        }
    }
    if txttnum != txcount {
        return errf!("block tx count need {} but got {}", txcount, txttnum)        
    }
    if txttsize > cnf.max_block_size { // may 1MB
        return errf!("block txs total size cannot over {} bytes", cnf.max_block_size)
    }
    // check mrkl root
    let mkroot = merge_mrkl_root(&txhxs);
    let mrklrt = block.mrklroot();
    if *mrklrt != mkroot {
        return errf!("block mrkl root need {} but got {}", mkroot, mrklrt)
    }
    // check mint checker and genesis , if consensus error
    mintk.consensus(&**block) ? ;
    // coinbase tx id = 0, if coinbase error
    mintk.coinbase(isrhei, &*alltxs[0]) ? ;
    // check state
    // fork new state
    let mut tempstate = fork_sub_state(this.state.upgrade().unwrap());
    // if init genesis status
    if isrhei == 1 {
        // set initialize 
        mintk.initialize(&mut tempstate) ? ;
    }
    // exec each tx
    // let txstabs = Arc::new(tempstate);
    for tx in alltxs.iter() {
        // let mut txstate = fork_sub_state(txstabs.clone());
        // kernel.vmobj.exec_tx( tx.to_readonly(), &mut tempstate) ? ;
        // ok merge copy state
        // tempstate.merge_copy(substate.as_ref()) ? ;
    }
    // 
    


    // ok return
    Ok((prevchunk.clone(), Arc::new(tempstate)))
}



