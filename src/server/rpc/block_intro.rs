

/******************* block intro *******************/



defineQueryObject!{ Q2953,
    height, Option<u32>, None,
    hash, Option<String>, None,
}

async fn block_intro(State(ctx): State<ApiCtx>, q: Query<Q2953>) -> impl IntoResponse {
    ctx_store!(ctx, store);
    q_unit!(q, unit);
    q_must!(q, hash, s!(""));
    q_must!(q, height, 0);
    // read
    let mut key = hash;
    if height > 0 {
        key = height.to_string();
    }
    let blkpkg = ctx.load_block(&store, &key);
    if let Err(_) = blkpkg {
        return api_error("cannot find block")
    }
    let blkpkg = blkpkg.unwrap();
    let blkobj = blkpkg.objc();
    
    // return data
    let txnum = blkobj.transaction_count().uint() - 1; // drop coinbase
    let mut data = jsondata!{
        "hash", blkpkg.hash().hex(),
        "height", blkobj.height().uint(),
        "timestamp", blkobj.timestamp().uint(),
        "mrklroot", blkobj.mrklroot().hex(),
        "prevhash", blkobj.prevhash().hex(),
        "transaction", txnum,
    };
    api_data(data)
}



/******************* block recents *******************/


defineQueryObject!{ Q7456,
    __nnn__, Option<u32>, None,
}

async fn block_recents(State(ctx): State<ApiCtx>, q: Query<Q7456>) -> impl IntoResponse {
    q_unit!(q, unit);
    let mut datalist = vec![];

    for li in  ctx.engine.recent_blocks() {
        datalist.push(jsondata!{
            "height", *li.height,
            "hash", li.hash.hex(),
            "prev", li.prev.hex(),
            "txs", *li.txs - 1,
            "miner", li.miner.readable(),
            "message", li.message.readable(),
            "reward", li.reward.to_unit_string(&unit),
            "time", *li.time,
            "arrive", *li.arrive,
        });
    }

    // ok
    api_data(jsondata!{
        "list", datalist,
    })
}