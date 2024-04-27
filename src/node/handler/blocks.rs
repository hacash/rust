

impl MsgHandler {

    async fn send_blocks(&self, peer: Arc<Peer>, mut buf: Vec<u8>) {
    }
    
    async fn receive_blocks(&self, peer: Arc<Peer>, mut buf: Vec<u8>) {
        if buf.len() < 3 * 8 {
            println!("check data failed.");
            return
        }
        let blocks = buf.split_off(3*8);
        let latest_hei = u64::from_be_bytes( bufcut!(buf, 0, 8) );
        let start_hei = u64::from_be_bytes( bufcut!(buf, 8, 16) );
        let end_hei = u64::from_be_bytes( bufcut!(buf, 16, 24) );
        let persent =  end_hei as f64 / latest_hei as f64 * 100.0;
        flush!("{}({:.2}%) inserting...", end_hei, persent);
        // try insert
        let engine = self.engine.clone();
        let res = tokio::task::spawn_blocking(move || {
            try_insert_blocks(engine, start_hei, blocks)
        }).await.unwrap();
        if "ok" != &res {
            return
        }
        println!("ok.");
        if end_hei >= latest_hei {
            println!("all blocks sync finished.");
            return
        }
        // sync more blocks
        let peer = self.switch_peer(peer);
        send_req_block_msg(peer, end_hei+1).await
    }


}

// return res msg
fn try_insert_blocks(engine: Arc<BlockEngine>, start_hei: u64, mut blocks: Vec<u8>) -> Error {
    if let Err(e) = engine.insert_sync(start_hei, blocks) {
        return e.to_string()
    }
    // ok
    "ok".to_string()
}



/*
// return res msg
fn try_insert_blocks_bnk(engine: Arc<BlockEngine>, start_hei: u64, mut blocks: Vec<u8>) -> String {
    let (blkist, blkch) = std::sync::mpsc::sync_channel(10);
    // loop to parse
    let handler = std::thread::spawn(move || {
        let mut hei = start_hei;
        let mut blocks = blocks.as_mut_slice();
        loop {
            // println!("blockdata {} = \n{}", hei, hex::encode(&blocks[..300]));
            if blocks.len() == 0 {
                break // end
            }
            let blk = protocol::block::create(&blocks);
            if let Err(e) = blk {
                print_sync_warning(format!("blocks::create error: {}", e));
                break
            }
            let (blk, sk) = blk.unwrap();
            let blkhei = blk.height().uint();
            if hei != blkhei {
                print_sync_warning(format!("need block height {} but got {}", hei, blkhei));
                break
            }
            // 
            let (left, right) = blocks.split_at_mut(sk);
            blocks = right; // next chunk
            let pkg = BlockPackage::new_with_data(blk, left.into());
            if let Err(_) = blkist.send(pkg) {
                break // end
            }
            hei += 1;
            // next block
        }
        // drop and close sender
    });
    // loop to insert
    loop {
        let blk = blkch.recv();
        if blk.is_err() {
            break // end
        }
        let blk = blk.unwrap();
        // do insert
        if let Err(e) = engine.insert(Box::new(blk)) {
            print_sync_warning(format!("engine.insert error: {}", e));
            return e
        }
    }
    // finish success
    "ok".to_string()
}



fn print_sync_warning(e: String) {
    println!("\n\n[Block Sync Warning] {}\n\n", e);
}

*/