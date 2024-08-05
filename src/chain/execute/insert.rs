
/**
 * do insert block crate new state
 * return new state
 */
pub fn do_check_insert(

    cnf: &EngineConf, 
    mintk: &dyn MintChecker, 
    store: &dyn Store,
    prev_state: Arc<ChainState>,
    prev_block: &dyn Block,
    blkpkg: &dyn BlockPkg

) -> Ret<ChainState> {

    let not_fast_sync = !cnf.fast_sync || blkpkg.origin() != block::BLOCK_ORIGIN::SYNC;

    // check height
    let block = blkpkg.objc();
    // let prev_block = prev_block;
    let height = block.height().to_u64();
    let blkhash = block.hash();
    let prev_height = prev_block.height().to_u64();
    if height != prev_height + 1 {
        return errf!("block height {} is not match to insert, need {}", height, prev_height + 1)
    }
    let alltxs = block.transactions();

    // is fast sync ignore all checks
    if not_fast_sync {
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
        let blk_size = blkpkg.body().length();
        if blk_size > cnf.max_block_size + 100 { // may 1MB + headmeta size
            return errf!("block size cannot over {} bytes", cnf.max_block_size + 100)
        }
        // check tx count
        let is_hash_with_fee = true;
        let txhxs = block.transaction_hash_list(is_hash_with_fee); // hash with fee
        let txcount = block.transaction_count().to_usize();
        if txcount < 1 {
            return err!("block txs cannot empty, need coinbase tx")
        }
        if txcount > cnf.max_block_txs { // may 1000
            return errf!("block txs cannot more than {}", cnf.max_block_txs)
        }
        if txcount != txhxs.len() {
            return errf!("block tx count need {} but got {}", txhxs.len(), txcount)
        }
        // check tx total size and count
        let mut txttsize = 0usize;
        let mut txttnum = 0usize;
        for tx in alltxs {
            txttnum += 1;
            txttsize += tx.size();
            if txttnum <= 1 { // coinbase not check
                continue
            }
            // check time
            if tx.timestamp().to_u64() > cur_time {
                return errf!("tx timestamp {} cannot more than now {}", tx.timestamp(), cur_time)
            }
            // verify signs
            transaction::verify_tx_signature(tx.as_ref().as_read())?; 
        }
        if txttnum != txcount {
            return errf!("block tx count need {} but got {}", txcount, txttnum)        
        }
        if txttsize > cnf.max_block_size { // may 1MB
            return errf!("block txs total size cannot over {} bytes", cnf.max_block_size)
        }
        // check mrkl root
        let mkroot = block::calculate_mrklroot(&txhxs);
        let mrklrt = block.mrklroot();
        if *mrklrt != mkroot {
            return errf!("block mrkl root need {} but got {}", mkroot, mrklrt)
        }
        // check mint consensus & coinbase
        mintk.consensus(store, prev_block.as_read(), block.as_read())?;
        // coinbase tx id = 0, if coinbase error
        let coinbase_tx = &*alltxs[0];
        mintk.coinbase(height, coinbase_tx)?;
    }

    // ready exec
    let coinbase_tx = &*alltxs[0];
    let mut alltxfee = Amount::default();
    // check state
    let mut sub_state = fork_sub_state(prev_state.clone());
    // if init genesis status
    if height == 1 {
        // state initialize 
        mintk.initialize(&mut sub_state)?;
    }
    // exec each tx
    let mut execn = 0;
    for tx in alltxs {
        if execn > 0 { // except coinbase tx
            exec_tx_actions(!not_fast_sync, height, blkhash, &mut sub_state, store, tx.as_read())?;
            alltxfee = alltxfee.add(&tx.fee_got())?; // fee_miner_received
        }
        // deduct tx fee after exec all actions
        tx.execute(height, &mut sub_state)?; // coinbase and other tx
        execn += 1;
    }
    // add miner got fee
    if alltxfee.is_positive() { // amt > 0
        let miner = coinbase_tx.address().unwrap();
        let mut corestate = CoreState::wrap(&mut sub_state);
        operate::hac_add(&mut corestate, &miner, &alltxfee)?;
    }
    // test
    Ok(sub_state)

}

