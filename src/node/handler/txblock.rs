

async fn handle_new_tx(this: Arc<MsgHandler>, peer: Arc<Peer>, body: Vec<u8>) {
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
    let p2p = this.p2pmng.lock().unwrap();
    let p2p = p2p.as_ref().unwrap();
    p2p.broadcast_message(1/*delay*/, knowkey, MSG_TX_SUBMIT, txpkg.body().clone().into_bytes());

}


async fn handle_new_block(this: Arc<MsgHandler>, peer: Arc<Peer>, body: Vec<u8>) {
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
    let p2p = this.p2pmng.lock().unwrap();
    let p2p = p2p.as_ref().unwrap();
    p2p.broadcast_message(1/*delay*/, knowkey, MSG_BLOCK_DISCOVER, blkpkg.body().clone().into_bytes());
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