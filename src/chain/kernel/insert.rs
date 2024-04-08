

impl Kernel for BlockChainKernel {

    fn insert(&self, blkpkg: Box<dyn BlockPkg>) -> RetErr {    
        self.isrlck.lock();
        // lock
        let rollres;
        {
            let ctx = self.klctx.lock().unwrap();
            // do insert
            let (bsck, state) = do_insert(self, &self.cnf, &ctx, self.mintk.as_ref(), blkpkg.as_ref()) ? ;
            // insert success try do roll
            rollres = do_roll(&self.cnf, &ctx, blkpkg, bsck, state) ? ;
        }
        if let Some((scusp, state, sroot)) = rollres {
            // change ptr
            let mut ctx = self.klctx.lock().unwrap();
            do_roll_chunk_state(&mut ctx, scusp, state, sroot) ? ;
        }
        // ok finish 
        Ok(())
    }

}

/**
 * do insert block crate new state
 */
fn do_insert(kernel: &BlockChainKernel, cnf: &KernelConf, this: &StateRoller, mintk: &dyn MintChecker, blkpkg: &dyn BlockPkg) -> Ret<(Arc<RollChunk>, Arc<ChainState>)> {

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
    let prevchunk = chunk::locate_base_chunk(this, prevhx);
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
    mintk.coinbase(&*alltxs[0]) ? ;
    // check state
    // fork new state
    let mut tempstate = fork_temp_state(this.state.upgrade().unwrap());
    // if init genesis status
    if isrhei == 1 {
        // genesis init error
        mintk.genesis(&mut tempstate) ? ;
    }
    // exec each tx
    // let txstabs = Arc::new(tempstate);
    for tx in alltxs.iter() {
        // let mut txstate = fork_temp_state(txstabs.clone());
        kernel.vmobj.exec_tx( tx.to_readonly(), &mut tempstate) ? ;
        // ok merge copy state
        // tempstate.merge_copy(substate.as_ref()) ? ;
    }
    // 
    


    // ok return
    Ok((prevchunk.clone(), Arc::new(tempstate)))
}