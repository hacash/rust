
defineQueryObject!{ Q4396,
    hex, Option<bool>, None,
}

async fn submit_transaction(State(ctx): State<ApiCtx>, q: Query<Q4396>, body: Bytes) -> impl IntoResponse {
    // body bytes
    let bddts = q_data_may_hex!(q, body.to_vec());
    // println!("get tx body: {}", hex::encode(&bddts));
    // parse
    let txpkg = transaction::create_pkg( BytesW4::from_vec(bddts) );
    if let Err(e) = txpkg {
        return api_error(&format!("transaction parse error: {}", &e))
    }
    let txpkg = txpkg.unwrap();
    // try submit
    let is_async = true;
    if let Err(e) = ctx.hcshnd.submit_transaction(&txpkg, is_async) {
        return api_error(&e)
    }
    // ok
    api_ok()
}