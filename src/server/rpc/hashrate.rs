
use crate::mint::difficulty::*;



fn query_hashrate(ctx: &ApiCtx) -> JsonObject {
    ctx_store!(ctx, store);
    ctx_state!(ctx, state);

    let mtckr = ctx.engine.mint_checker();
    let mtcnf = mtckr.config();
    let btt = mtcnf.each_block_target_time; // 300
    let bac = mtcnf.difficulty_adjust_blocks; // 288
    //
    let lastblk = ctx.engine.latest_block();
    let lastblk = lastblk.objc();
    let curhei = *lastblk.height();
    let tg_difn = lastblk.difficulty().uint();
    let mut tg_hash = u32_to_hash(tg_difn);
    let tg_rate = hash_to_rates(&tg_hash, btt); // 300sec
    let tg_show = rates_to_show(tg_rate);
    // 
    let mut rt_rate = tg_rate;
    let mut rt_show = tg_show.clone();
    let ltc = 100u64; // realtime by current 100 blocks 
    if curhei > ltc {
        let key = (curhei - ltc).to_string();
        let pblk = ctx.load_block(&store, &key);
        if let Ok(pblk) = pblk {
            let p100t = pblk.objc().timestamp().uint();
            let cttt = (lastblk.timestamp().uint() - p100t) / ltc;
            rt_rate = rt_rate * btt as u128 / cttt as u128;
            rt_show = rates_to_show(rt_rate);
        }
    }
    
    // return data
    right_00_to_ff(&mut tg_hash);
    let mut data = jsondata!{
        "target", jsondata!{
            "rate", tg_rate,
            "show", tg_show,
            "hash", hex::encode(&tg_hash),
            "difn", tg_difn, // difficulty number
        },
        "realtime", jsondata!{
            "rate", rt_rate,
            "show", rt_show,
        },
    };

    data
}





defineQueryObject!{ Q5295,
    __nnn_, Option<bool>, None,
}

async fn hashrate(State(ctx): State<ApiCtx>, q: Query<Q5295>) -> impl IntoResponse {

    let data = query_hashrate(&ctx);

    api_data(data)
}



defineQueryObject!{ Q9314,
    days, Option<u64>, None,
    target, Option<bool>, None,
    scale, Option<f64>, None,
}

async fn hashrate_logs(State(ctx): State<ApiCtx>, q: Query<Q9314>) -> impl IntoResponse {
    ctx_store!(ctx, store);
    ctx_state!(ctx, state);
    q_must!(q, days, 200);
    q_must!(q, target, false);
    q_must!(q, scale, 0.0);

    let mtckr = ctx.engine.mint_checker();
    let bac = mtckr.config().difficulty_adjust_blocks; // 300
    //
    if days > 500 {
        return api_error("param days cannot more than 500")
    }
    let lasthei = ctx.engine.latest_block().objc().height().uint();
    if lasthei < days {
        return api_error("param days overflow")
    }
    let secs = lasthei / days;

    // load list
    let mx = days as usize;
    let mut day200 = Vec::with_capacity(mx);
    let mut dayall = Vec::with_capacity(mx);
    let mut day200_max = 0u128;
    let mut dayall_max = 0u128;
    for i in 0..days {
        let s1 = lasthei - ((days-1-i) * bac);
        let s2 = secs + secs*i;
        // println!("{} {}", s1, s2);
        let rt1 = get_blk_rate(&ctx, &store, s1);
        let rt2 = get_blk_rate(&ctx, &store, s2);
        if rt1 > day200_max {
            day200_max = rt1;
        }
        if rt2 > dayall_max {
            dayall_max = rt2;
        }
        day200.push(rt1);
        dayall.push(rt2);
    }

    // scale
    if scale > 0.0 {
        let mut sd2 = day200_max as f64 / scale;
        let mut sda = dayall_max as f64 / scale;
        for i in 0..day200.len() {
            let v = day200[i] as f64;
            day200[i] = (v / sd2) as u128;
        }
        for i in 0..dayall.len() {
            let v = dayall[i] as f64;
            dayall[i] = (v / sda) as u128;
        }
    }

    let mut data = JsonObject::new();

    // realtime & target
    if target {
        data = query_hashrate(&ctx);
    }

    // return data
    data.insert("day200", json!(day200));
    data.insert("dayall", json!(dayall));

    api_data(data)
}



////////////////////////



fn get_blk_rate(ctx: &ApiCtx, store: &CoreStoreDisk, hei: u64) -> u128 {
    let key = hei.to_string();
    let difn = ctx.load_block(store, &key).unwrap().objc().difficulty().uint();
    let mtckr = ctx.engine.mint_checker();
    u32_to_rates(difn, mtckr.config().each_block_target_time) // 300s
}


/*
fn drop_right_ff(hx: &[u8]) -> Vec<u8> {
    let mut res = vec![];
    for a in hx {
        if *a < 255 {
            res.push(*a);
        }else{
            break;
        }
    }
    res
}
*/


fn right_00_to_ff(hx: &mut [u8]) {
    let m = hx.len();
    for i in 0..hx.len() {
        let n = m - i - 1;
        if hx[n] == 0 { // 00
            hx[n] = 255; // ff
        }else{
            break // finish
        }
    }
}


