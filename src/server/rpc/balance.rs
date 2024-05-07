use crate::mint::component::DiamondOwnedForm;


defineQueryObject!{ Q8364,
    address, String, s!(""),
}

async fn balance(State(ctx): State<ApiCtx>, q: Query<Q8364>) -> impl IntoResponse  {
    ctx_state!(ctx, state);
    ctx_mintstate!(ctx, mintstate);
    q_unit!(q, unit);
    let ads = q.address.replace(" ","").replace("\n","");
    let addrs: Vec<_> = ads.split(",").collect();
    let adrsz = addrs.len();
    if adrsz == 0 || (adrsz==1 && addrs[0].len()==0) {
        return api_error("address format error")
    }
    if adrsz > 200 {
        return api_error("address max 200")
    }
    let mut resbls = Vec::with_capacity(adrsz);
    for a in addrs {
        let adr = Address::form_readable(a);
        if let Err(e) = adr {
            return api_error(&format!("address {} format error", a))
        }
        let adr = adr.unwrap();
        // balance
        let bls = state.balance(&adr).unwrap_or_default();
        // dianames
        let diaowned = mintstate.diamond_owned(&adr).unwrap_or_default();
        resbls.push(json!({
            "hacash": bls.hacash.to_unit_string(&unit),
            "diamond": bls.diamond.uint(),
            "satoshi": bls.satoshi.uint(),
            "diamonds": diaowned.readable(),
        }));
    }
    // ok
    api_data_list(resbls)
}