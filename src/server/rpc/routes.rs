

pub fn routes(mut ctx: ApiCtx) -> Router {

    use ctx::*;
    
    let lrt = Router::new().route("/", get(console))
    
    // query
    .route(&query("latest"), get(latest))
    .route(&query("supply"), get(supply))
    .route(&query("hashrate"), get(hashrate))
    .route(&query("hashrate/logs"), get(hashrate_logs))
    .route(&query("balance"), get(balance))
    .route(&query("channel"), get(channel))
    .route(&query("coin/transfer"), get(scan_coin_transfer))

    .route(&query("block/intro"), get(block_intro))
    .route(&query("block/recents"), get(block_recents))
    .route(&query("block/views"), get(block_views))
    .route(&query("block/datas"), get(block_datas))

    .route(&query("transaction/exist"), get(transaction_exist))
    .route(&query("transaction/check"), get(transaction_check))
    .route(&query("transaction/build"), get(transaction_build))
    .route(&query("transaction/sign"), get(transaction_sign))

    .route(&query("diamond"), get(diamond))
    .route(&query("diamond/bidding"), get(diamond_bidding))
    .route(&query("diamond/views"), get(diamond_views))

    .route(&query("miner/notice"), get(miner_notice))
    .route(&query("miner/pending"), get(miner_pending))

    // create
    .route(&create("account"), get(account))
    .route(&create("coin/transfer"), get(create_coin_transfer))
    
    // submit
    .route(&submit("transaction"), post(submit_transaction))
    .route(&submit("block"), post(submit_block))
    .route(&submit("miner/success"), get(miner_success))

    // operate
    .route(&operate("fee/raise"), post(raise_fee));

    // merge unstable & extend
    Router::new().merge(lrt)
    .merge(unstable::routes())
    .merge(extend::routes())
    .with_state(ctx)
    
}



