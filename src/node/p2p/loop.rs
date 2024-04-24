

impl P2PManage {

    pub async fn event_loop(&self) -> RetErr {
        let mut reconnect_tkr = new_ticker(60*15).await; // 15mins check reconnect
        let mut findnodes_tkr = new_ticker(60*60*4).await; // 4hour find nodes or boot
        let mut checkpeer_tkr = new_ticker(60*3).await; // 3mins pin all no active nodes

        loop {
            tokio::select! {
                _ = reconnect_tkr.tick() => {
                    let no_nodes = self.backbones().len() < 2;
                    if no_nodes && self.cnf.find {
                        self.connect_boot_nodes().await; // connect boots
                    }
                },
                _ = findnodes_tkr.tick() => {
                    if self.cnf.find {
                        self.find_nodes().await; // do find nodes
                    }
                },
                _ = checkpeer_tkr.tick() => {
                    self.check_active_nodes().await; // do check no active
                    self.ping_nodes().await; // do ping all nodes
                    self.boost_public();
                },
                else => break
            }
            // println!("loop next");
        }
        println!("[P2P] event loop end.");
        Ok(())
    }

}

