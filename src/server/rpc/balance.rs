
async fn balances(State(ctx): State<ApiCtx>, req: Request) -> Json<Value>  {

    Json(json!([{"hacash":"12:248","diamond":0,"satoshi":0}]))
}