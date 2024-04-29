
async fn balances(State(ctx): State<ApiCtx>, req: Request) -> String  {
    ctx_state!(ctx, state);
    let rwds = state.block_reward();

    "".to_string()

    // Json(json!([{"hacash":"12:248","diamond":0,"satoshi":0}]))
}