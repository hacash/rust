

pub fn routes(mut ctx: ApiCtx) -> Router {
    Router::new().route("/", get(console))

    // query
    .route(&query("balance"), get(balance))
    .route(&query("coin_transfer"), get(scan_coin_transfer))

    // create
    .route(&create("account"), get(account))
    .route(&create("coin_transfer"), get(create_coin_transfer))
    

    // submit
    // ...

    // ctx
    .with_state(ctx)
}




// paths
fn query(p: &str) -> String {
    "/query/".to_owned() + p
}
fn create(p: &str) -> String {
    "/create/".to_owned() + p
}
fn submit(p: &str) -> String {
    "/submit/".to_owned() + p
}