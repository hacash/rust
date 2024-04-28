

async fn console(State(ctx): State<ApiCtx>, req: Request) -> String {

    let latest = ctx.engine.latest_block();
    let latest = latest.objc();

    // render
    format!(r#"
        <h1>Hacash server console</h1>
        <p>latest block height: {}</p>
    "#, latest.height().uint())
}

