

pub fn routes(mut ctx: ApiCtx) -> Router {
    Router::new().route("/", get(console))

    // query
    .route(&query("balances"), get(balances))

    // submit
    // ...

    // ctx
    .with_state(ctx)
}




// paths
fn query(p: &str) -> String {
    "/query/".to_owned() + p
}
fn submit(p: &str) -> String {
    "/submit/".to_owned() + p
}