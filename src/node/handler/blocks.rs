

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
            engine.insert_sync(start_hei, blocks)
        }).await.unwrap();
        if let Err(e) = res {
            println!("{}", e); // show err
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
