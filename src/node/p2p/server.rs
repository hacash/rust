

impl P2PManage {

    /*
    pub async fn server_listen(this: Arc<P2PManage>) -> RetErr {

        let port = this.cnf.listen;
        let listener = errunbox!( TcpListener::bind( format!("127.0.0.1:{}", port) ).await )?;
        loop {
            let (client, _) = errunbox!( listener.accept().await )?;
            let tobj = this.clone();
            tokio::spawn(async move {
                tobj.handle_conn(client, false).await // not report me
            });
        }
        println!("P2PManage server listen loop end.");

        Ok(())
    }
    */

    async fn server(&self) -> TcpListener {

        let port = self.cnf.listen;
        let listener = TcpListener::bind( format!("127.0.0.1:{}", port) ).await.unwrap();
        listener

    }

}
