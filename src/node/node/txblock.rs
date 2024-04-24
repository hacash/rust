/**
* Tx Block Arrive Msg
*/

impl HacashNode {

    fn handle_txblock_arrive(mut self) {

        loop {
            println!("do_handle_txblock_arrive loop next");
            match self.blktxch.recv().unwrap() {
                BlockTxMsgStuff::Tx(peer, tx) => 
                    println!("handle_txblock_arrive Tx, peer={} len={}", peer.nick(), tx.len()),
                BlockTxMsgStuff::Block(peer, blk) => 
                    println!("handle_txblock_arrive Block, peer={} len={}", peer.nick(), blk.len()),
            }
            std::thread::sleep(secs(1));
        }
        println!("Hacash node loop end.");
    }


}


