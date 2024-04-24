

impl P2PManage {

    pub async fn event_loop(&self) -> RetErr {
        let mut reconnect_tkr = new_ticker(51*15).await; // 15mins check reconnect
        let mut findnodes_tkr = new_ticker(52*60*4).await; // 4hour find nodes or boot
        let mut checkpeer_tkr = new_ticker(53*3).await; // 3mins ping all no active nodes
        let mut boostndes_tkr = new_ticker(54*5).await; // 5mins boost public nodes form offshoots table
        loop {
            tokio::select! {
                _ = reconnect_tkr.tick() => {
                    let no_nodes = self.backbones().len() < 2;
                    if no_nodes && self.cnf.findnodes {
                        self.connect_boot_nodes().await; // connect boots
                    }
                },
                _ = findnodes_tkr.tick() => {
                    if self.cnf.findnodes {
                        self.find_nodes().await; // do find nodes
                    }
                },
                _ = checkpeer_tkr.tick() => {
                    self.check_active_nodes().await; // do check no active
                    self.ping_nodes().await; // do ping all nodes
                },
                _ = boostndes_tkr.tick() => {
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

