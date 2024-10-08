

/******************* diamondminer init *******************/


defineQueryObject!{ Q7846,
    ___nnn___, Option<bool>, None,
}

async fn diamondminer_init(State(ctx): State<ApiCtx>, q: Query<Q7846>) -> impl IntoResponse {
    // ctx_mintstate!(ctx, mintstate);
    // q_must!(q, wait, 45); // 45 sec
    let cnf = ctx.engine.config();

    if ! cnf.dmer_enable {
        return api_error("diamond miner in config not enable");
    }

    let data = jsondata!{
        "bid_address", cnf.dmer_bid_account.readable(),
        "reward_address", cnf.dmer_reward_address.readable(),
    };

    api_data(data)
}


/******************* diamondminer success *******************/



defineQueryObject!{ Q6396,
    ___nnn___, Option<bool>, None,
}

async fn diamondminer_success(State(ctx): State<ApiCtx>, q: Query<Q6396>, body: Bytes) -> impl IntoResponse {
    ctx_mintstate!(ctx, mintstate);
    // q_must!(q, wait, 45); // 45 sec
    let cnf = ctx.engine.config();

    if ! cnf.dmer_enable {
        return api_error("diamond miner in config not enable");
    }

    let actdts = q_body_data_may_hex!(q, body);
    let Ok((mint, _)) = DiamondMint::create(&actdts) else {
        return api_error("upload action error");
    };

    let mint_number = mint.head.number.uint();
    let mint_name = mint.head.diamond.readable();

    // check number and hash
    let lastdia = mintstate.latest_diamond();
    if mint_number != lastdia.number.uint() + 1 {
        return api_error("diamond number error");
    }
    if mint_number > 1 && mint.head.prev_hash != lastdia.born_hash {
        return api_error("diamond prev hash error");
    }

    // create trs
    let bid_addr = Address::cons(cnf.dmer_bid_account.address().clone());
    let mut tx = TransactionType2::build(bid_addr, cnf.dmer_bid_min.clone());
    tx.push_action(Box::new(mint));
    tx.fill_sign(&cnf.dmer_bid_account);

    let txhx = tx.hash();

    // add to tx pool
    let txpkg: Box<dyn TxPkg> = Box::new(TxPackage::new(Box::new(tx)));
    // try submit
    let in_async = true;
    if let Err(e) = ctx.hcshnd.submit_transaction(&txpkg, in_async) {
        return api_error(&e)
    }

    let hxstr = txhx.hex();
    println!("▒▒▒▒ DIAMOND SUCCESS: {}({}), tx hash: {}.", mint_name, mint_number, &hxstr);

    let data = jsondata!{
        "tx_hash", hxstr,
    };

    api_data(data)
}