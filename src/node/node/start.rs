

impl HacashNode {

    // 
    pub fn start(&mut self) -> RetErr {


        /*
        let rt = self.tokiort.take().unwrap();
        rt.spawn(async{
            // connect boot node
            let addr = "127.0.0.1:3337".parse().unwrap();
            let socket = TcpSocket::new_v4().unwrap();
            let mut conn = socket.connect(addr).await.unwrap();
            // send handshake
            let handshake = 3418609257u32.to_be_bytes();
            AsyncWriteExt::write_all(&mut conn, &handshake).await;

            let nodeinfo = hex::decode("0000999911111111111111111111111111111111ffffffffffffffffffffffffffffffff").unwrap();
            tcp_send_msg(&mut conn, 1, nodeinfo.to_vec()).await;
            println!("send nodeinfo len = {}", nodeinfo.len());

            // create peer
            let mut peer = super::p2p::Peer::create(addr, conn).await.unwrap();
            peer.is_public = true;
            println!("peer {:?}", peer);

        });
        self.tokiort = Some(rt);
        */


        // 
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









