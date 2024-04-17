

impl HacashNode {

    // 
    pub fn start(&mut self) -> RetErr {

        let t1 = thread::spawn(||{
            let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
            // let rt = Runtime::new().unwrap();
            thread::sleep(Duration::from_secs(1));
            println!("thread::sleep(Duration::from_secs(1));");

            rt.block_on(async {
                println!("before sleep: {}", Local::now().format("%F %T.%3f"));
                tokio::time::sleep(Duration::from_secs(2)).await;
                println!("after sleep: {}", Local::now().format("%F %T.%3f"));
            });
        });




        thread::sleep(Duration::from_secs(5));
        println!("main::sleep(Duration::from_secs(5));");
        Ok(())
    }

}









