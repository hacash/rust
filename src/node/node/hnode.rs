

impl HNode for HacashNode {

    fn submit_transaction(&self, txpkg: &Box<dyn TxPkg>, in_async: bool) -> RetErr {
        // try execute tx
        self.engine.try_execute_tx(txpkg.objc().as_ref().as_read())?;
        // add to pool
        let msghdl = self.msghdl.clone();
        let txbody = txpkg.body().clone().into_vec();
        let runobj = async move {
            msghdl.submit_transaction(txbody).await;
        };
        if in_async {
            tokio::spawn(runobj);
        }else{
            new_current_thread_tokio_rt().block_on(runobj);
        }
        Ok(())
    }


    fn submit_block(&self, blkpkg: &Box<dyn BlockPkg>, in_async: bool) -> RetErr {
        // NOT do any check
        // insert
        let msghdl = self.msghdl.clone();
        let blkbody = blkpkg.body().clone().into_vec();
        let runobj = async move {
            msghdl.submit_block(blkbody).await;
        };
        if in_async {
            tokio::spawn(runobj);
        }else{
            new_current_thread_tokio_rt().block_on(runobj);
        }
        Ok(())
    }

    fn engine(&self) -> Arc<dyn Engine> {
        self.engine.clone()
    }

    fn txpool(&self) -> Arc<dyn TxPool> {
        self.txpool.clone()
    }

    fn all_peer_prints(&self) -> Vec<String> { 
        self.p2p.all_peer_prints()
    }
}
