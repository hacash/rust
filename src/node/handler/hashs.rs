

impl MsgHandler {

    async fn send_hashs(&self, peer: Arc<Peer>, mut buf: Vec<u8>) {


    }

    async fn receive_hashs(&self, peer: Arc<Peer>, mut buf: Vec<u8>) {
        // println!("receive_hashs = {}", hex::encode(&buf));
        if buf.len() < 8 {
            println!("check hash failed.");
            return
        }
        let hashs = buf.split_off(8);
        let end_hei = u64::from_be_bytes( bufcut!(buf, 0, 8) );
        let hash_len = hashs.len();
        if hash_len == 0 || hash_len % 32 != 0 {
            return // error len
        }
        let mut hash_num = hash_len as u64 / 32;
        // check
        let latest = self.engine.latest_block();
        let lathei = latest.objc().height().uint();
        if end_hei > lathei {
            return // not find target height block
        }
        let dfhmax = self.engine.config().unstable_block as u64; 
        if hash_num > dfhmax {
            hash_num = dfhmax;
        }
        let mut start_hei = end_hei - hash_num;
        if end_hei <= hash_num {
            start_hei = 1; // first block
        }
        // diff each blk hash
        let stoptr = self.engine.store();
        let store = CoreStoreDisk::wrap(stoptr.as_ref());
        let mut hi = 0;
        for hei in (start_hei..=end_hei).rev() {
            let myhx = store.blockptr(&BlockHeight::from(hei));
            if myhx.is_none() {
                return // not find block hash by height
            }
            let myhx = myhx.unwrap();
            let hx = Fixed32::cons( bufcut!(hashs, hi, hi+32) );
            if hx == myhx {
                // sync blocks from next height
                try_sync_blocks(self, peer, hei + 1).await;
                return // to sync new blocks
            }
            // next
            hi += 32;
        }
        // cannot sync fork!!!
    }

}




