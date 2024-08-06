

defineQueryObject!{ Q3735,
    hex, Option<bool>, None,
}

async fn submit_block(State(ctx): State<ApiCtx>, q: Query<Q3735>, body: Bytes) -> impl IntoResponse {
    // body bytes
    let bddts = q_data_may_hex!(q, body.to_vec());
    // println!("get block body: {}", hex::encode(&bddts));
    // parse
    let blkpkg = block::create_pkg( BytesW4::from_vec(bddts) );
    if let Err(e) = blkpkg {
        return api_error(&format!("block parse error: {}", &e))
    }
    let blkpkg = blkpkg.unwrap();
    // try submit
    let is_async = true;
    if let Err(e) = ctx.hcshnd.submit_block(&blkpkg, is_async) {
        return api_error(&format!("submit block error: {}", &e))
    }
    // ok
    api_ok()
}