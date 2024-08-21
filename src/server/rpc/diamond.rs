

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
        "inscripts", diaobj.inscripts.array(),
        // smelt
        "number", diasmelt.number.uint(),
        // "miner", diasmelt.miner_address.readable(),
        "height", diasmelt.belong_height.uint(), // belong block height
        "bid_fee", diasmelt.bid_fee.to_unit_string(&unit),
        "life_gene", diasmelt.life_gene.hex(),
        "visual_gene", calculate_diamond_visual_gene(&dian, &diasmelt.life_gene).hex(),
    };
    api_data(data)
}


/******************* diamond bidding *******************/


defineQueryObject!{ Q8346,
    limit, Option<usize>, None,
    number, Option<usize>, None,
}

async fn diamond_bidding(State(ctx): State<ApiCtx>, q: Query<Q8346>) -> impl IntoResponse {
    ctx_mintstate!(ctx, mintstate);
    let lastdia = mintstate.latest_diamond();
    q_unit!(q, unit);
    q_must!(q, limit, 20);
    q_must!(q, number, 0);
    let number = number as u32;

    let mut datalist = vec![];
    // load from txpool
    let txpool = ctx.hcshnd.tx_pool();

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
            return true // number not match
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

    // return data
    api_data(jsondata!{
        "number", *lastdia.number + 1, // next diamond
        "list", datalist,
    })
}



/******************* diamond bidding *******************/


defineQueryObject!{ Q5395,
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
    if limit > 200 {
        limit = 200;
    }

    // load by list
    let mut datalist = vec![];

    // ids
    let diarng = get_id_range(lastdianum, page, limit, start, desc);
    // println!("{:?}", diarng);
    for id in diarng {
        let Some(dian) = mintstate.diamond_ptr(&DiamondNumber::from(id as u32)) else {
            continue
        };
        let Some(diaobj) = mintstate.diamond(&dian) else {
            continue
        };
        let Some(diasmelt) = mintstore.diamond_smelt(&dian) else {
            continue
        };
        let data = jsondata!{
            "number", id,
            "name", dian.readable(),
            "bid_fee", diasmelt.bid_fee.to_unit_string(&unit),
            "life_gene", diasmelt.life_gene.hex(),
            // "visual_gene", calculate_diamond_visual_gene(&dian, &diasmelt.life_gene).hex(),
        };
        datalist.push(data);
    }

    // return data
    api_data(jsondata!{
        "latest_number", lastdianum,
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
