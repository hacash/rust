

impl HacashNode {

    pub fn event_loop(&mut self) -> RetErr {
        // start loop
        let rt = self.tokiort.take().unwrap();
        let this = self;
        rt.block_on(async{
            do_event_loop(this).await
        });
        // end
        Ok(())
    }
}


async fn do_event_loop(node: &mut HacashNode)-> RetErr {

    // println!("mk={} node.cnf.node_key = {}", node.mk, hex::encode(node.cnf.node_key));


    asleep(15).await;

    let mut exp_tkr = new_ticker(1, 4000).await;
    let mut find_tkr = new_ticker(1, 1000).await;


    loop {
        tokio::select! {

            _ = exp_tkr.tick() => println!("exp_tkr +++"),
            _ = find_tkr.tick() => println!("find_tkr !"),
            else => {
                println!("loop else break");
                break;
            }

        }
        println!("loop next ----------------------");
    }


    println!("tokio loop end");

    Ok(())
}


