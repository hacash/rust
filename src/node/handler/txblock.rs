

async fn handle_new_tx(this: Arc<MsgHandler>, peer: Arc<Peer>, body: Vec<u8>) {
    // println!("1111111 handle_txblock_arrive Tx, peer={} len={}", peer.nick(), body.clone().len());
    // parse
    let txpkg = transaction::create_pkg(BytesW4::from_vec_u8(body));
    if let Err(e) = txpkg {
        return // parse tx error
    }
    let txpkg = txpkg.unwrap();
    // tx hash with fee
    let hxfe = txpkg.objc().hash_with_fee();
    let (already, knowkey) = check_know(&this.knows, &peer.knows, &hxfe);
    if already {
        return  // alreay know it
    }

    // TODO:: append to txpool


    // broadcast
    let p2p = this.p2pmng.lock().unwrap();
    let p2p = p2p.as_ref().unwrap();
    p2p.broadcast_message(1/*delay*/, knowkey, MSG_TX_SUBMIT, txpkg.body().clone().into_bytes());

}


async fn handle_new_block(this: Arc<MsgHandler>, peer: Arc<Peer>, body: Vec<u8>) {
    // println!("222222222222 handle_txblock_arrive Block, peer={} len={}", peer.nick(), body.clone().len());
    let mut blkhead = BlockIntro::new();
    if let Err(_) = blkhead.parse(&body, 0) {
        return // parse tx error
    }
    let blkhei = blkhead.height().uint();
    let blkhx = blkhead.hash();
    let (already, knowkey) = check_know(&this.knows, &peer.knows, &blkhx);
    if already {
        return  // alreay know it
    }
    // check height and difficulty (mint consensus)
    let heispan = this.engine.config().unstable_block;
    let latest = this.engine.latest_block();
    let lathei = latest.objc().height().uint();
    if blkhei < lathei - heispan {
        return // height too late
    }
    let mintckr = this.engine.mint_checker();
    let stoptr = this.engine.store();
    if let Err(_) = mintckr.prepare(stoptr.as_ref(), &blkhead) {
        return  // difficulty check fail
    }
    if blkhei <= lathei + 1 {
        // do insert  ◆ ◇ ⊙ ■ □ △ ▽ ❏ ❐ ❑ ❒  ▐ ░ ▒ ▓ ▔ ▕ ■ □ ▢ ▣ ▤ ▥ ▦ ▧ ▨ ▩ ▪ ▫    
        let hxtail = &blkhx.as_bytes()[24..];
        let txs = blkhead.transaction_count().uint() - 1;
        let blkts = &timeshow(blkhead.timestamp().uint())[11..];
        print!("❏  discover block {} …{} txs{:2} time {} inserting at {} ... ", 
            blkhei, hex::encode(hxtail), txs, blkts, &ctshow()[11..]);
        let bodycp = body.clone();
        let engicp = this.engine.clone();
        std::thread::spawn(move||{
            // create block
            let blkpkg = block::create_pkg(BytesW4::from_vec_u8(bodycp));
            if let Err(e) = blkpkg {
                return // parse error
            }
            if let Err(e) = engicp.insert(blkpkg.unwrap()) {
                println!("Error: {}", e)
            }else{
                println!("ok.")
            }
        });
    }else{
        // req sync
        send_req_block_hash_msg(peer, (heispan+1) as u8, lathei).await;
        return // not broadcast
    }
    // broadcast new block
    let p2p = this.p2pmng.lock().unwrap();
    let p2p = p2p.as_ref().unwrap();
    p2p.broadcast_message(1/*delay*/, knowkey, MSG_BLOCK_DISCOVER, body);
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