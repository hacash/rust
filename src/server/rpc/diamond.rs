

/******************* diamond *******************/



defineQueryObject!{ Q3946,
    name, Option<String>, None,
    number, Option<u32>, None,
}

async fn diamond(State(ctx): State<ApiCtx>, q: Query<Q3946>) -> impl IntoResponse {
    ctx_mintstate!(ctx, mintstate);
    ctx_mintstore!(ctx, mintstore);
    q_unit!(q, unit);
    q_must!(q, name, s!(""));
    q_must!(q, number, 0);
    // id
    if number > 0 {
        let dian = mintstate.diamond_ptr(&DiamondNumber::from(number));
        if let None = dian {
            return api_error("cannot find diamond")
        }
        name = dian.unwrap().readable();
    }else if !DiamondName::is_valid(&name.as_bytes()) {
        return api_error("diamond name error")
    }
    // data
    let dian = DiamondName::cons(name.as_bytes().try_into().unwrap());
    let diaobj = mintstate.diamond(&dian);
    if let None = diaobj {
        return api_error("cannot find diamond")
    }
    let diaobj = diaobj.unwrap();
    // load smelt
    let diasmelt = mintstore.diamond_smelt(&dian);
    if let None = diasmelt {
        return api_error("cannot find diamond")
    }
    let diasmelt = diasmelt.unwrap();
    // return data
    let mut data = jsondata!{
        "name", dian.readable(),
        "belong", diaobj.address.readable(),
        "inscriptions", diaobj.inscripts.array(),
        // smelt
        "number", diasmelt.number.uint(),
        "miner", diasmelt.miner_address.readable(),
        "born", jsondata!{
            "height", diasmelt.born_height.uint(), // born block height
            "hash", diasmelt.born_hash.hex(), // born block hash
        },
        "prev_hash", diasmelt.prev_hash.hex(),
        "bid_fee", diasmelt.bid_fee.to_unit_string(&unit),
        "average_bid_burn", diasmelt.average_bid_burn.uint(),
        "life_gene", diasmelt.life_gene.hex(),
        "visual_gene", calculate_diamond_visual_gene(&dian, &diasmelt.life_gene).hex(),
    };
    api_data(data)
}


/******************* diamond bidding *******************/


defineQueryObject!{ Q8346,
    limit, Option<usize>, None,
    number, Option<usize>, None,
    since, Option<bool>, None,
}

async fn diamond_bidding(State(ctx): State<ApiCtx>, q: Query<Q8346>) -> impl IntoResponse {
    ctx_store!(ctx, store);
    ctx_mintstate!(ctx, mintstate);
    let lastdia = mintstate.latest_diamond();
    q_unit!(q, unit);
    q_must!(q, limit, 20);
    q_must!(q, number, 0);
    q_must!(q, since, false);
    let number = number as u32;

    let mut datalist = vec![];
    // load from txpool
    let txpool = ctx.hcshnd.txpool();

    // loop diamond mit tx
    let mut pick_dmint = |a: &Box<dyn TxPkg>| {

        if datalist.len() >= limit {
            return false // end
        }
        let txhx = a.hash();
        let txr = a.objc().as_ref().as_read();
        let Some(diamtact) = pickout_diamond_mint_action(txr) else {
            return true // continue
        };
        let act = diamtact.head;
        if number > 0 && number != act.number.uint() {
            return true // number not match, continue
        }
        // append
        let mut one = jsondata!{
            // "purity", a.fee_purity(),
            "tx", txhx.hex(),
            "fee", txr.fee().to_unit_string(&unit),
            "bid", txr.address().unwrap().readable(),
            "name", act.diamond.readable(),
            "belong", act.address.readable(),
        };
        if number == 0 {
            one.insert("number", json!(*act.number));
        }
        datalist.push(one);
        true // next

    };
    txpool.iter_at(&mut pick_dmint, TXPOOL_GROUP_DIAMOND_MINT);

    let mut data = jsondata!{
        "number", *lastdia.number + 1, // current bidding diamond
        "list", datalist,
    };

    if since {
        let mut acution_start = curtimes(); 
        if let Ok(blk) = ctx.load_block( &store, &lastdia.born_height.to_string() ) {
            acution_start = blk.objc().timestamp().uint();
            data.insert("since", json!(acution_start));
        }
    }

    // return data
    api_data(data)
}



/******************* diamond views *******************/


defineQueryObject!{ Q5395,
    name, Option<String>, None,
    limit, Option<i64>, None,
    page, Option<i64>, None,
    start, Option<i64>, None,
    desc, Option<bool>, None,
}

async fn diamond_views(State(ctx): State<ApiCtx>, q: Query<Q5395>) -> impl IntoResponse {
    ctx_mintstore!(ctx, mintstore);
    ctx_mintstate!(ctx, mintstate);
    let lastdianum = *mintstate.latest_diamond().number as i64;
    q_unit!(q, unit);
    q_must!(q, limit, 20);
    q_must!(q, page, 1);
    q_must!(q, start, i64::MAX);
    q_must!(q, desc, false);
    q_must!(q, name, s!(""));

    if limit > 200 {
        limit = 200;
    }

    // load by list
    let mut datalist = vec![];

    let mut query_item = |dian: &DiamondName|{
        let Some(diaobj) = mintstate.diamond(dian) else {
            return
        };
        let Some(diasmelt) = mintstore.diamond_smelt(dian) else {
            return
        };
        let data = jsondata!{
            "name", dian.readable(),
            "number", diasmelt.number.uint(),
            "bid_fee", diasmelt.bid_fee.to_unit_string(&unit),
            "life_gene", diasmelt.life_gene.hex(),
            // "visual_gene", calculate_diamond_visual_gene(&dian, &diasmelt.life_gene).hex(),
        };
        datalist.push(data);
    };

    // read diamonds
    if name.len() >= DiamondName::width() {

        let Ok(names) = DiamondNameListMax200::from_readable(&name) else {
            return api_error("diamond name error")
        };
        for dian in names.list() {
            query_item(&dian);
        }

    }else{

        // ids
        let diarng = get_id_range(lastdianum, page, limit, start, desc);
        // println!("{:?}", diarng);
        for id in diarng {
            let Some(dian) = mintstate.diamond_ptr(&DiamondNumber::from(id as u32)) else {
                continue
            };
            query_item(&dian);
        }
    }

    // return data
    api_data(jsondata!{
        "latest_number", lastdianum,
        "list", datalist,
    })
}




/******************* diamond engrave *******************/


defineQueryObject!{ Q5733,
    height, u64, 0,
    txposi, Option<isize>, None, // -1,
    tx_hash, Option<bool>, None, // if return txhash
}

async fn diamond_engrave(State(ctx): State<ApiCtx>, q: Query<Q5733>) -> impl IntoResponse {
    ctx_store!(ctx, store);
    ctx_mintstate!(ctx, mintstate);
    q_unit!(q, unit);
    q_must!(q, tx_hash, false);
    q_must!(q, txposi, -1);

    let mut datalist = vec![];

    // load block
    let blkpkg = ctx.load_block(&store, &q.height.to_string());
    if let Err(e) = blkpkg {
        return api_error(&e)
    }
    let blkobj = blkpkg.unwrap();
    let blkobj = blkobj.objc();
    let trs = blkobj.transactions();
    if trs.len() == 0 {
        return api_error("transaction len error")
    }
    if txposi >= 0 {
        if txposi >= trs.len() as isize - 1 {
            return api_error("txposi overflow")
        }
    }

    // parse
    let pick_engrave = |tx: &dyn TransactionRead| -> Option<Vec<_>> {
        let mut res = vec![];
        let txhx = tx.hash();
        let mut append_one = |data: JsonObject| {
            let mut engobj = data;
            if tx_hash {
                engobj.insert("tx_hash", json!(txhx.hex()));
            }
            res.push(json!(engobj));
        };
        for act in tx.actions() {
            if act.kind() == DiamondInscription::kid() {
                let action = DiamondInscription::must(&act.serialize());
                append_one(jsondata!{
                    "diamonds", action.diamonds.readable(),
                    "inscription", action.engraved_content.readable_or_hex(),
                });
            }else if act.kind() == DiamondInscriptionClear::kid() {
                let action = DiamondInscriptionClear::must(&act.serialize());
                append_one(jsondata!{
                    "diamonds", action.diamonds.readable(),
                    "clear", true,
                });
            }
        }
        Some(res)
    };

    let tx_ary = match txposi >= 0 {
        true => { let i=txposi as usize; &trs[1..][i..i+1] },
        false => &trs[1..],
    };

    // ignore coinbase tx
    for tx in tx_ary {
        if let Some(mut egrs) = pick_engrave(tx.as_read()) {
            datalist.append(&mut egrs);
        }
    }

    // return data
    api_data(jsondata!{
        "list", datalist,
    })
}



/*****************************************/


fn pickout_diamond_mint_action(tx: &dyn TransactionRead) -> Option<DiamondMint> {
    let mut res: Option<DiamondMint> = None;
    for a in tx.actions() {
        if a.kind() == ACTION_KIND_ID_DIAMOND_MINT {
            let act = DiamondMint::must(&a.serialize());
            res = Some(act);
            break // find ok
        }
    }
    res
}
