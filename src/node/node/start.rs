

impl HacashNode {

    // 
    pub fn start(mut self) -> RetErr {

        let rt = self.tokiort.take().unwrap();
        let p2p = self.p2p.clone();

        // handle msg
        rt.spawn_blocking(move ||{
            self.handle_txblock_arrive();
        });
        // do_event_loop(this).await

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