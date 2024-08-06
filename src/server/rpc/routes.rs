

pub fn routes(mut ctx: ApiCtx) -> Router {

    use ctx::*;
    
    let lrt = Router::new().route("/", get(console))
    
    // query
    .route(&query("latest"), get(latest))
    .route(&query("supply"), get(supply))
    .route(&query("hashrate"), get(hashrate))
    .route(&query("hashrate/logs"), get(hashrate_logs))
    .route(&query("balance"), get(balance))
    .route(&query("diamond"), get(diamond))
    .route(&query("block/intro"), get(block_intro))
    .route(&query("coin/transfer"), get(scan_coin_transfer))

    // create
    .route(&create("account"), get(account))
    .route(&create("coin/transfer"), get(create_coin_transfer))
    
    // submit
    .route(&submit("transaction"), post(submit_transaction))
    .route(&submit("block"), post(submit_block))

    // operate
    .route(&operate("fee/raise"), post(raise_fee));

    // merge unstable & extend
    Router::new().merge(lrt)
    .merge(unstable::routes())
    .merge(extend::routes())
    .with_state(ctx)
    
}



