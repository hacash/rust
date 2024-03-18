

impl Kernel for BlockChainKernel {

    fn insert(&mut self, blkpkg: &dyn BlockPkg) -> Option<Error> {
        impl_insert(self, blkpkg)
    }

}


// do insert
fn impl_insert(this: &mut BlockChainKernel, blkpkg: &dyn BlockPkg) -> Option<Error> {
    // lock
    this.isrlck.lock();
    // check height
    let block = blkpkg.objc();
    let isrhei = block.height().to_u64();
    let rthei = this.sroot.height.to_u64();
    if isrhei <= rthei {
        return erf!("block height {} is too low to insert, need above {}", isrhei, rthei)
    }
    let underhei = rthei + this.cnf.unstable_block + 1; // unstable block = 4
    if isrhei > underhei {
        return erf!("block height {} is too high to insert, need equal or under {}", isrhei, underhei)
    }
    // check prev hash
    let prevhx = block.prevhash();
    let prevchunk = locate_base_chunk(this, prevhx);
    let Some(prevchunk) = prevchunk else {
        return erf!("cannot find base block by prev hash {}", prevhx)
    };
    let prevhei = prevchunk.height.to_u64();
    if prevhei+1 != isrhei {
        return erf!("block height need {} but got {}", prevhei+1, isrhei)
    }
    // check tx count
    let txhxs = block.transaction_hash_list();
    let txcount = block.transaction_count() as usize;
    if txcount < 1 {
        return er!("block txs cannot empty, need coinbase tx")
    }
    if txcount > this.cnf.max_block_txs { // may 999
        return erf!("block txs cannot more than {}", this.cnf.max_block_txs)
    }
    if txcount != txhxs.len() {
        return erf!("block tx count need {} but got {}", txhxs.len(), txcount)
    }
    // check size
    let blksz = blkpkg.body().len();
    if blksz > this.cnf.max_block_size { // may 1MB
        return erf!("block size cannot over {} bytes", this.cnf.max_block_size)
    }
    // check mrkl root
    let mkroot = merge_mrkl_root(&txhxs);
    let mrklrt = block.mrklroot();
    if *mrklrt != mkroot {
        return erf!("block mrkl root need {} but got {}", mkroot, mrklrt)
    }
    // check time
    let prevblkts = prevchunk.block.objc().timestamp().to_u64();
    let blkts = block.timestamp().to_u64();
    let ctis = curtimes();
    if blkts > ctis {
        return erf!("block timestamp {} cannot more than system timestamp {}", blkts, ctis)
    }
    if blkts < prevblkts {
        return erf!("block timestamp {} cannot less than prev block {}", blkts, prevblkts)
    }
    // 



    



    None
}