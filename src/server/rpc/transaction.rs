

/******************* transaction desc *******************/



defineQueryObject!{ Q3457,
    hash, String, s!(""),
    actions, Option<bool>, None,
}

async fn transaction_desc(State(ctx): State<ApiCtx>, q: Query<Q3457>) -> impl IntoResponse {
    ctx_store!(ctx, store);
    ctx_state!(ctx, state);
    q_unit!(q, unit);
    q_must!(q, actions, false);

    let Ok(hx) = hex::decode(&q.hash) else {
        return api_error("transaction hash format error")
    };
    if hx.len() != 32 {
        return api_error("transaction hash format error")
    }
    let txhx = Hash::must(&hx);
    let Some(txp) = state.txexist(&txhx) else {
        return api_error("transaction not find")
    };
    // read block
    let bkey = txp.height.to_string();
    let blkpkg = ctx.load_block(&store, &bkey);
    if let Err(_) = blkpkg {
        return api_error("cannot find block by transaction ptr")
    }
    let blkpkg = blkpkg.unwrap();
    let blkobj = blkpkg.objc();
    let blktrs = blkobj.transactions();

    // search tx hash
    let tx = match (|hx: &Hash|{
        let txnum = blkobj.transaction_count().uint() as usize; // drop coinbase
        for i in 1..txnum {
            if *hx == blktrs[i].hash() {
                return Some(blktrs[i].clone())
            }
        }
        None
    })(&txhx) {
        None => return api_error("transaction not find in the block"),
        Some(tx) => tx,
    };

    let mut data = jsondata!{
        // block
        "block", jsondata!{
            "height", blkobj.height().uint(),
            "timestamp", blkobj.timestamp().uint(),
        },
        // tx
        "type", tx.timestamp().uint(),
        "timestamp", tx.timestamp().uint(),
        "fee", tx.fee().to_unit_string(&unit),
        "fee_got", tx.fee_got().to_unit_string(&unit),
        "main_address", tx.address().unwrap().readable(),
        "action", tx.action_count(),
    };

    if actions {
        let txread = tx.as_read();
        let acts = tx.actions();
        let mut actobjs = Vec::with_capacity(acts.len());
        for act in acts {
            actobjs.push( action_json_desc(txread, act.as_ref(), &unit, true) );
        }
        data.insert("actions", json!(actobjs));
    }

    api_data(data)
}


