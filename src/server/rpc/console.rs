

async fn console(State(ctx): State<ApiCtx>, req: Request) -> impl IntoResponse {
    ctx_store!(ctx, store);

    let mtcnf = ctx.engine.mint_checker().config().clone();

    let latest = ctx.engine.latest_block();
    let latest = latest.objc();
    let lathei = latest.height().uint() as i64;
    let latts = latest.timestamp().uint();

    // target time
    let cyln = mtcnf.difficulty_adjust_blocks as i64;
    let secnp = ["day", "week", "month", "quarter", "year", "all"];
    let secn = [cyln, cyln*7, cyln*30, cyln*90, cyln*365, lathei-1];
    let mut target_time = Vec::with_capacity(secn.len());
    
    for i in 0..secn.len() {
        let sb = secn[i];
        let hei = lathei - sb;
        if hei <= 0 {
            break
        }
        let blkdts = store.blockdatabyptr(&BlockHeight::from(hei as u64));
        if blkdts.is_none() {
            break
        }
        let mut bhd = BlockIntro::default();
        if let Err(..) = bhd.parse(blkdts.unwrap().as_ref(), 0) {
            break
        }
        let blkt = bhd.timestamp().uint();
        target_time.push(format!(
            "{}: {}s", secnp[i], (latts-blkt)/(sb as u64),
        ));
    }


    // render
    (html_headers(), format!(r#"<html><head><title>Hacash node console</title></head><body>
        <h3>Hacash console</h3>
        <p>Latest height {} time {}</p>
        <p>Block span times: {}</p>
        <p>{}</p>
    </body></html>"#, 
        latest.height().uint(),
        timeshow(latest.timestamp().uint()),
        target_time.join(", "),
        ctx.hcshnd.tx_pool().print(),
    ))
}

