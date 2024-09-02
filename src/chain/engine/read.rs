
impl EngineRead for BlockEngine {

    fn config(&self) -> &EngineConf {
        &self.cnf
    }

    fn state(&self) -> Arc<dyn State> {
        self.klctx.lock().unwrap().state.upgrade().unwrap()
    }

    fn store(&self) -> Arc<dyn Store> {
        self.store.clone()
    }

    /*
    fn confirm_state(&self) -> (Arc<dyn State>, Arc<dyn BlockPkg>) {
        let roller = self.klctx.lock().unwrap();
        let rtstate = roller.sroot.state.clone();
        let rtblkpkg = roller.sroot.block.clone();
        (rtstate, rtblkpkg)
    }
    */

    fn latest_block(&self) -> Arc<dyn BlockPkg> {
        let curctx = self.klctx.lock().unwrap();
        let curchk = curctx.scusp.upgrade().unwrap();
        curchk.block.clone()
    }

    fn mint_checker(&self) -> Arc<dyn MintChecker> {
        self.mintk.clone().into()
    }

    fn try_execute_tx(&self, tx: &dyn TransactionRead) -> RetErr {
        let sta = self.get_latest_state();
        if let None = sta {
            return errf!("block engine not yet")
        }
        let mut sub_state = fork_sub_state(sta.unwrap());
        let height = self.get_latest_height().uint() + 1; // next height
        let blkhash = Hash::cons([0u8; 32]); // empty hash
        // exec
        exec_tx_actions(false, self.cnf.chain_id, height, blkhash, &mut sub_state, self.store.as_ref(), tx)?;
        tx.execute(height, &mut sub_state)
    } 

    fn recent_blocks(&self) -> Vec<Arc<RecentBlockInfo>> {
        let vs = self.rctblks.lock().unwrap();   
        let res: Vec<_> = vs.iter().map(|x|x.clone()).collect();
        res
    }

    /* 
    * 1w zhu(shuo) / 200byte
    * 
    */
    fn average_fee_purity(&self) -> u64 {
        let avgfs = self.avgfees.lock().unwrap();
        let al = avgfs.len();
        if al == 0 {
            return DEFAULT_AVERAGE_FEE_PURITY
        }
        let mut allfps = 0u64;
        for a in avgfs.iter() {
            allfps += a;
        }
        allfps / al as u64
    }


}
