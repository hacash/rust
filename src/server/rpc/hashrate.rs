
use crate::mint::difficulty::*;


const BLOCK_TARGET_TIME: u64 = 300;  // seconds
const BLOCK_ADJUST_CYCLE: u64 = 288; // blocks


defineQueryObject!{ Q5295,
    __nnn_, Option<bool>, None,
}

async fn hashrate(State(ctx): State<ApiCtx>, q: Query<Q5295>) -> impl IntoResponse {
    ctx_store!(ctx, store);
    ctx_state!(ctx, state);
    let btt = BLOCK_TARGET_TIME;
    let bac = BLOCK_ADJUST_CYCLE;
    //
    let lastblk = ctx.engine.latest_block();
    let lastblk = lastblk.objc();
    let curhei = *lastblk.height();
    let tg_difn = lastblk.difficulty().uint();
    let tg_hash = u32_to_hash(tg_difn);
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
    let mut data = jsondata!{
        "target", jsondata!{
            "rate", tg_rate,
            "show", tg_show,
            "hash", hex::encode(drop_right_ff(&tg_hash)),
            "difn", tg_difn, // difficulty number
        },
        "realtime", jsondata!{
            "rate", rt_rate,
            "show", rt_show,
        },
    };
    api_data(data)
}



defineQueryObject!{ Q9314,
    days, Option<u64>, None,
}

async fn hashrate_logs(State(ctx): State<ApiCtx>, q: Query<Q9314>) -> impl IntoResponse {
    ctx_store!(ctx, store);
    ctx_state!(ctx, state);
    q_must!(q, days, 200);
    let bac = BLOCK_ADJUST_CYCLE;
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
    for i in 0..days {
        let s1 = lasthei - ((days-1-i) * bac);
        let s2 = secs + secs*i;
        // println!("{} {}", s1, s2);
        day200.push(get_blk_rate(&ctx, &store, s1));
        dayall.push(get_blk_rate(&ctx, &store, s2));
    }

    // return data
    let mut data = jsondata!{
        "day200", day200,
        "dayall", dayall,
    };
    api_data(data)

}


////////////////////////

fn get_blk_rate(ctx: &ApiCtx, store: &CoreStoreDisk, hei: u64) -> u128 {
    let key = hei.to_string();
    let difn = ctx.load_block(store, &key).unwrap().objc().difficulty().uint();
    u32_to_rates(difn, BLOCK_TARGET_TIME)
}

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

