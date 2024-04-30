

async fn console(State(ctx): State<ApiCtx>, req: Request) -> impl IntoResponse {

    let latest = ctx.engine.latest_block();
    let latest = latest.objc();

    // target time
    let mut target_time = "".to_string();



    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/html".parse().unwrap());
    // render
    (headers, format!(r#"<html><head><title>Hacash node console</title></head><body>
        <h1>Hacash console</h1>
        <p>latest height {} time {}</p>
        <p>{}</p>
    </body></html>"#, 
        latest.height().uint(),
        timeshow(latest.timestamp().uint()),
        target_time,
    ))
}

