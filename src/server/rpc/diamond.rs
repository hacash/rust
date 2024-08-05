

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
        "mint_height", diasmelt.belong_height.uint(),
        "bid_fee", diasmelt.bid_fee.to_unit_string(&unit),
        "life_gene", diasmelt.life_gene.hex(),
        "visual_gene", calculate_diamond_visual_gene(&dian, &diasmelt.life_gene).hex(),
    };
    api_data(data)
}

