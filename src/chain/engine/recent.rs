


impl BlockEngine {


    fn record_recent(&self, block: &dyn BlockRead) {
        if !self.cnf.recent_blocks {
            return // config not open it
        }
        let chei = block.height().uint() as i128;
        let deln = (self.cnf.unstable_block * 2) as i128;
        let deln = chei - deln;
        // delete
        let mut rcts = self.rctblks.lock().unwrap();
        rcts.retain(|x| x.height.uint() as i128 > deln);
        // insert
        let rctblk = create_recent_block_info(block);
        rcts.push_front(rctblk.into()); // arc
    }



}