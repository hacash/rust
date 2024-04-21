

impl HacashNode {

    // 
    pub fn start(&mut self) -> RetErr {

        let rt = self.tokiort.as_mut().unwrap();

        // start p2p listen
        let p2p = self.p2p.clone();
        rt.spawn(async move{
            P2PManage::start_listen(p2p).await
        });

        // connect boot nodes
        let p2p = self.p2p.clone();
        rt.spawn(async move{
            // asleep(1).await;
            p2p.connect_boot_nodes().await
        });

        // start event loop
        self.event_loop()
    }

}









