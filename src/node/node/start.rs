

impl HacashNode {

    // 
    pub fn start(&mut self) -> RetErr {

        let rt = self.tokiort.as_mut().unwrap();

        // start p2p listen
        let p2p = self.p2p.clone();
        rt.spawn(async move{
            p2p.start_listen().await
        });

        // connect boot nodes
        let p2p = self.p2p.clone();
        rt.spawn(async move{
            // asleep(1).await;
            p2p.connect_boot_nodes().await
        });

        // start event loop
        self.event_loop()

        
        /*
        // let t1 = thread::spawn(||{
            let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
            // let rt = Runtime::new().unwrap();
            thread::sleep(Duration::from_secs(1));
            println!("thread::sleep(Duration::from_secs(1));");
e
            rt.block_on(async {
                println!("before sleep: {}", Local::now().format("%F %T.%3f"));
                tokio::tim::sleep(Duration::from_secs(2)).await;
                println!("after sleep: {}", Local::now().format("%F %T.%3f"));
            });
        // });




        thread::sleep(Duration::from_secs(5));
        println!("main::sleep(Duration::from_secs(5));");
        Ok(())
        */
    }

}









