/**
* Tx Block Arrive Msg
*/

impl HacashNode {

    fn handle_txblock_arrive(this: Arc<HacashNode>, mut blktxch: Receiver<BlockTxArrive>) {
        let rt = new_current_thread_tokio_rt();
        // run loop
        rt.block_on(async move {
            let mut closech = { 
                this.closech.lock().unwrap().take().unwrap()
            };
            loop {
                tokio::select! {
                    _ = closech.recv() => {
                        break
                    },
                    msg = blktxch.recv() => {
                        match msg.unwrap() {
                            BlockTxArrive::Tx(peer, tx) => handle_new_tx(this.clone(), peer, tx).await,
                            BlockTxArrive::Block(peer, blk) => handle_new_block(this.clone(), peer, blk).await,
                        }
                    }
                }
            }
            // println!("Hacash node txblock arrive loop end.");
        });

    }

}


async fn handle_new_tx(this: Arc<HacashNode>, peer: Arc<Peer>, body: Vec<u8>) {
    // println!("1111111 handle_txblock_arrive Tx, peer={} len={}", peer.nick(), body.clone().len());
    // parse
    let txpkg = transaction::create_pkg(BytesW4::from_vec_u8(body));
    if let Err(e) = txpkg {
        return // parse tx error
    }
    let txpkg = txpkg.unwrap();
    // tx hash with fee
    let (already, knowkey) = check_know(&this.knows, &peer.knows, &txpkg.objc().hash_with_fee());
    if already {
        return  // alreay know it
    }

    // TODO::

    // broadcast
    tokio::spawn(async move {
        asleep(1).await; // sleep to avoid duplicate broadcasts
        this.p2p.broadcast_unaware(&knowkey, MSG_TX_SUBMIT, txpkg.body().clone().into_bytes()).await;
        // println!("handle_txblock_arrive Tx, peer={} hx={}", peer.nick(), txpkg.hash());
    });
}




async fn handle_new_block(this: Arc<HacashNode>, peer: Arc<Peer>, body: Vec<u8>) {
    // println!("222222222222 handle_txblock_arrive Block, peer={} len={}", peer.nick(), body.clone().len());
    let blkpkg = block::create_pkg(BytesW4::from_vec_u8(body));
    if let Err(e) = blkpkg {
        return // parse tx error
    }
    let blkpkg = blkpkg.unwrap();
    let (already, knowkey) = check_know(&this.knows, &peer.knows, blkpkg.hash());
    if already {
        return  // alreay know it
    }

    // TODO::

    // broadcast
    tokio::spawn(async move {
        asleep(1).await; // sleep to avoid duplicate broadcasts
        this.p2p.broadcast_unaware(&knowkey, MSG_BLOCK_DISCOVER, blkpkg.body().clone().into_bytes()).await;
        // println!("handle_txblock_arrive Block, peer={} hash={}", peer.nick(), blkpkg.hash());
    });
}



// return already know
fn check_know(mine: &Knowledge, peer: &Knowledge, hxkey: &Hash) -> (bool, KnowKey) {
    let knowkey: [u8; KNOWLEDGE_SIZE] = hxkey.clone().into_array();
    peer.add(knowkey.clone());
    if mine.check(&knowkey) {
        return (true, knowkey) // alreay know it
    }
    mine.add(knowkey.clone());
    (false, knowkey)
}