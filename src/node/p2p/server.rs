

impl P2PManage {

    async fn server(&self) -> TcpListener {

        let port = self.cnf.listen;
        let listener = TcpListener::bind( format!("127.0.0.1:{}", port) ).await.unwrap();
        listener

    }

}
