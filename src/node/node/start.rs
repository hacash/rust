

impl HacashNode {


    
    // 
    pub fn init(mut self) -> (Arc<HacashNode>, Receiver<BlockTxArrive>) {
        let chrx = self.blktxch.take().unwrap();
        ( Arc::new(self), chrx)
    }

    // 
    pub fn start(this: Arc<HacashNode>, chrx: Receiver<BlockTxArrive> ) -> RetErr {

        let rt = new_current_thread_tokio_rt();
        let p2p = this.p2p.clone();

        // handle msg
        let hn2 = this.clone();
        rt.spawn_blocking(move||{
            HacashNode::handle_txblock_arrive(hn2, chrx);
        });

        // start p2p loop, blocking
        rt.block_on(async{
            P2PManage::start(p2p).await
        });

        // ret
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