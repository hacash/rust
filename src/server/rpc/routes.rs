

pub fn routes(mut ctx: ApiCtx) -> Router {

    use ctx::*;
    
    let lrt = Router::new().route("/", get(console))
    
    // query
    .route(&query("latest"), get(latest))
    .route(&query("balance"), get(balance))
    .route(&query("diamond"), get(diamond))
    .route(&query("block_intro"), get(block_intro))
    .route(&query("coin_transfer"), get(scan_coin_transfer))

    // create
    .route(&create("account"), get(account))
    .route(&ctx::create("coin_transfer"), get(create_coin_transfer))
    
    // submit
    .route(&submit("transaction"), post(submit_transaction))
    .route(&submit("block"), post(submit_block));

    // operate
    // //

    // merge unstable & extend
    Router::new().merge(lrt)
    .merge(unstable::routes())
    .merge(extend::routes())
    .with_state(ctx)
    
}



