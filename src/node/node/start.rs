

impl HacashNode {

    // 
    pub fn start(mut self) -> RetErr {

        let rt = self.tokiort.take().unwrap();
        let chrx = self.blktxch.take().unwrap();
        let p2p = self.p2p.clone();

        // handle msg
        let node = Arc::new(self);
        rt.spawn_blocking(move||{
            HacashNode::handle_txblock_arrive(node, chrx);
        });

        // start p2p loop, blocking
        rt.block_on(async{
            P2PManage::start(p2p).await
        });

        // end
        Ok(())
        
    }

}











/*


        // test
        for n in 0..256u8 {
            let b = 100;
            println!("{}<->{}: {}", b, n, super::p2p::calculate_one_byte_topology_distance(b, n));
        }
        panic!("{}", "test end");



*/