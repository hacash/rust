

fn _test_blocks() -> Vec<BlockV1> {
    let datas = vec![
        "010000000001005c57b130000000077790ba2fcdeaef4a4299d9b667135bac577ce204dee8388f1b97f7e64448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f30000000100000516fffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
        "010000000002005c57b2e6001e231cb03f9938d54f04407797b8188f0375eb10f0bcb426dccae87dcadb564448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f300000001000007adfffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
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

    let blks = _test_blocks();

    for blk in blks {
        // engine.print_roller();
        let pkg = BlockPackage::new(Box::new(blk));
        if let Err(e) = engine.insert(Box::new(pkg)) {
            println!("{}", e);
        }
    }

    engine.print_roller();


}