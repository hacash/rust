

defineQueryObject!{ Q5396,
    hex, Option<bool>, None,
    fee, String, s!(""),
    fee_prikey, String, s!(""),
    tx_hash, String, s!(""),
}

async fn raise_fee(State(ctx): State<ApiCtx>, q: Query<Q5396>, body: Bytes) -> impl IntoResponse {
    // ctx_store!(ctx, store);

    let fee = q_data_amt!(q, fee);
    let acc = q_data_acc!(q, fee_prikey);
    
    // body bytes
    let bddts = q_data_may_hex!(q, body.to_vec());
    // parse
    let txb = transaction::create(&bddts);
    if let Err(e) = txb {
        return api_error(&format!("transaction parse error: {}", &e))
    }
    let (mut txb, _) = txb.unwrap();
    // check set fee
    let old_fee = txb.fee();
    if fee.less_than(old_fee) {
        return api_error(&format!("fee {} cannot less than {}", fee, old_fee))
    }
    txb.set_fee(fee.clone());
    txb.fill_sign(&acc);
    let txhash = txb.hash();
    let txhashwf = txb.hash_with_fee();
    // pkg
    let txpkg: Box<dyn TxPkg> = Box::new(TxPackage::new(txb));
    // submit tx & add to txpool
    let is_async = true;
    if let Err(e) = ctx.hcshnd.submit_transaction(&txpkg, is_async) {
        return api_error(&e)
    }
    // ok
    let data = jsondata!{
        "hash", txhash.hex(),
        "hash_with_fee", txhashwf.hex(),
        "fee", fee.to_fin_string(),
    };
    api_data(data)
}