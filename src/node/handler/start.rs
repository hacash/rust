


impl MsgHandler {

    pub fn start(this: Arc<MsgHandler>) {
        let rt = new_current_thread_tokio_rt();
        rt.block_on(async move {
            let mut closech = this.closer.signal();
            let mut blktxch = { 
                this.blktxch.lock().unwrap().take().unwrap()
            };
            loop {
                tokio::select! {
                    // close signal
                    _ = closech.recv() => {
                        break
                    },
                    // block tx arrived
                    msg = blktxch.recv() => {
                        match msg.unwrap() {
                            BlockTxArrive::Tx(peer, tx) => handle_new_tx(this.clone(), peer, tx).await,
                            BlockTxArrive::Block(peer, blk) => handle_new_block(this.clone(), peer, blk).await,
                        }
                    }
                }
            }
        })

    }
}



