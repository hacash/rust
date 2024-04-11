


fn _test_blocks() -> Vec<BlockV1> {
    let datas = vec![
        // hei: 1, hash: 001e231cb03f9938d54f04407797b8188f0375eb10f0bcb426dccae87dcadb56
        "010000000001005c57b130000000077790ba2fcdeaef4a4299d9b667135bac577ce204dee8388f1b97f7e64448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f30000000100000516fffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
        // hei: 2, hash: 000c0a2a3761fec7aa214975c1cce407b509a828d16dcf6d3bdb1f612a2466f5
        "010000000002005c57b2e6001e231cb03f9938d54f04407797b8188f0375eb10f0bcb426dccae87dcadb564448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f300000001000007adfffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",


        // hei: 1, hash: b3e25014147b2ee7232323259a86955a2779fa442d0ba86444de4eaa3f392a93
        "010000000001005c57b0ef000000077790ba2fcdeaef4a4299d9b667135bac577ce204dee8388f1b97f7e6ad557702fc70afaf70a855e7b8a4400159643cb5a7fc8a89ba2bce6f818a9b0100000001098b344500000000000000000c1aaa4e6007cc58cfb932052ac0ec25ca356183f80101686172646572746f646f62657474657200",
        // hei: 2, hash: 1e78eaa957a162e1b80caa856670fa0df3ba16e694a8edbfc057b71c15fd0ae6
        "010000000002005c5826ca001e231cb03f9938d54f04407797b8188f0375eb10f0bcb426dccae87dcadb56ad557702fc70afaf70a855e7b8a4400159643cb5a7fc8a89ba2bce6f818a9b0100000001098b344500000000000000000c1aaa4e6007cc58cfb932052ac0ec25ca356183f80101686172646572746f646f62657474657200",
        // hei: 3, hash: 3a11fee226605b7b760205d15c56c3699dcb25cf442ffe08124beda8fd3814a8
        "010000000003005c5826cb1e78eaa957a162e1b80caa856670fa0df3ba16e694a8edbfc057b71c15fd0ae6ad557702fc70afaf70a855e7b8a4400159643cb5a7fc8a89ba2bce6f818a9b0100000001098b344500000000000000000c1aaa4e6007cc58cfb932052ac0ec25ca356183f80101686172646572746f646f62657474657200",
        

        // hei: 3, hash: 0015920ecbd8048128b9e27a26bd08b488050c78b89291d740889ed4d785f410
        "010000000003005c57b3f3000c0a2a3761fec7aa214975c1cce407b509a828d16dcf6d3bdb1f612a2466f54448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f3000000010000037afffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
        // hei: 4, hash: 000e9a58542cc77a2442721d839c23be67549940dbe3f0e84e18fb1630d90ae4
        "010000000004005c57b52d0015920ecbd8048128b9e27a26bd08b488050c78b89291d740889ed4d785f4104448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f30000000100000039fffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
        // hei: 5, hash: 004fc675773657de765c1a355e9123624d7524bc3cdaaeea298a49c74583fe40
        "010000000005005c57b635000e9a58542cc77a2442721d839c23be67549940dbe3f0e84e18fb1630d90ae44448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f300000001000004ebfffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
        // hei: 6, hash: 001a0cad03d0ff1f8b6d8f7d9c1e35bbb1d648c50f16738b4a4029c9e4c5be79
        "010000000006005c57b69b004fc675773657de765c1a355e9123624d7524bc3cdaaeea298a49c74583fe404448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f300000001000005b7fffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
        // hei: 7, hash: 001a62c40e11085237574ab293c3056085f6071dff744c9883fd2c5b92f7f33e
        "010000000007005c57b742001a0cad03d0ff1f8b6d8f7d9c1e35bbb1d648c50f16738b4a4029c9e4c5be794448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f3000000010000076ffffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
        // hei: 8, hash: 003e7e42d2b866a066477894f9a14d788061265503ee144286c7d4927df3b3f4
        "010000000008005c57b825001a62c40e11085237574ab293c3056085f6071dff744c9883fd2c5b92f7f33e4448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f3000000010000013afffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
        // hei: 9, hash: 0012beb10c426427407d769c7c4ace246f1c564dabc7787c86ee5b21c3517cdb
        "010000000009005c57b983003e7e42d2b866a066477894f9a14d788061265503ee144286c7d4927df3b3f44448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f3000000010000041ffffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
    ];
    let mut blks = Vec::with_capacity(datas.len());
    for dt in datas {
        let dt = hex::decode(dt).unwrap();
        let mut blk = BlockV1::new();
        if let Err(e) = blk.parse(&dt, 0) {
            panic!("{}", e)
        }
        blks.push(blk);
    }
    blks
}







pub fn engine_test_1(engine: Arc<BlockEngine>) {

    // //  1549280970
    // create_block(3, 1549280971, "1e78eaa957a162e1b80caa856670fa0df3ba16e694a8edbfc057b71c15fd0ae6");
    // return
    


    engine.print_roller();

    let blks = _test_blocks();

    for blk in blks {

        println!("\n---------------------------------------\n");

        let pkg = BlockPackage::new(Box::new(blk));
        if let Err(e) = engine.insert(Box::new(pkg)) {
            println!("{}", e);
        }
        engine.print_roller();
    }

    // delete datadir
    std::fs::remove_dir_all("./hacash_mainnet_data");


}


pub fn create_block(height: u64, pt: u64, prev: &str) {

    let blktime = Timestamp::from_uint(pt); // 1549250700);
    let blknoncenum = Uint4::from_uint(160117829);
    let reward_addr = Address::form_readable(&"1271438866CSDpJUqrnchoJAiGGBFSQhjd".to_string()).unwrap();
    let mut trsvec = DynVecTransaction::new(); 
    trsvec.push(Box::new(TransactionCoinbase{
        ty: Uint1::from_uint(0),
        address: reward_addr,
        reward: Amount::new_coin(1),
        message: StringTrim16::from_readable(b"hardertodobetter"),
        extend: CoinbaseExtend::new()
    }));
    let mut genesis_block = BlockV1 {
        intro: BlockHeadMeta { 
            head: BlockHead { 
                version: Uint1::from_uint(1), 
                height: BlockHeight::from_uint(height), 
                timestamp: blktime, 
                prevhash: Hash::from_hex(prev.as_bytes()),
                mrklroot: Hash::new(), // 000000...
                transaction_count: Uint4::from_uint(1) // trs 1
            }, 
            meta: BlockMeta { 
                nonce: blknoncenum, 
                difficulty: Uint4::from_uint(0), 
                witness_stage: Fixed2::new() 
            },
        },
        transactions: trsvec
    };
    // set mrklroot
    genesis_block.update_mrklroot();

    
    println!("hash: {}", hex::encode(genesis_block.hash()));
    println!("body: {}", hex::encode(genesis_block.serialize()));


}