
defineQueryObject!{ Q4538,
    height, u64, 1,
    txposi, usize, 0,
}

async fn scan_coin_transfer(State(ctx): State<ApiCtx>, q: Query<Q4538>) -> impl IntoResponse {
    ctx_store!(ctx, store);
    q_unit!(q, unit);
    q_coinkind!(q, coinkind);
    let blkpkg = ctx.load_block(&store, &q.height.to_string());
    if let Err(e) = blkpkg {
        return  api_error(&e)
    }
    let blkobj = blkpkg.unwrap();
    let blkobj = blkobj.objc();
    let trs = blkobj.transactions();
    if trs.len() == 0 {
        return api_error("transaction len error")
    }
    let trs = &trs[1..];
    if q.txposi >= trs.len() {
        return api_error("txposi overflow")
    }
    let tartrs = &trs[q.txposi];
    let mut dtlist = json!([]);
    let dtlistptr = dtlist.as_array_mut().unwrap();
    // scan actions
    for act in tartrs.actions()  {
        append_transfer_scan(&unit, &coinkind, dtlistptr, act.as_ref());
    }
    // ok
    let mut data = jsondata!{
        "tx_hash", tartrs.hash().hex(),
        "block_hash", blkobj.hash().hex(),
        "tx_timestamp", tartrs.timestamp().uint(),
        "block_timestamp", blkobj.timestamp().uint(),
        "address", tartrs.address().unwrap().readable(),
        "transfers", dtlist,
    };
    api_data(data)
}




macro_rules! transfer_scan_action_item_old {
    ( $kid: expr, $act: expr, $transfers: expr, $actname: ident, $acty: ty, $ck: expr, $jsonobj: expr ) => (
        if $kid == <$acty>::kid(){
            if false == $ck {
                return
            }
            let $actname = <$acty>::build(&$act.serialize()).unwrap();
            $transfers.push($jsonobj);
            return
        }
    )
}


fn append_transfer_scan(unit: &str, ck: &CoinKind, transfers: &mut Vec<Value>, act: &dyn Action) {
    let kid = act.kind();

    macro_rules! transfer_scan_action_item {
        ( $actname: ident, $acty: ty, $ck: expr, $jsonobj: expr ) => (
            if kid == <$acty>::kid(){
                if false == $ck {
                    return
                }
                let $actname = <$acty>::build(&act.serialize()).unwrap();
                transfers.push($jsonobj);
                return
            }
        )
    }

    // HacTransfer // 1
    transfer_scan_action_item!( act, HacTransfer, ck.hacash, json!({
        "to": act.to.readable(),
        "hacash": act.amt.to_unit_string(unit),
    }));
    // HacFromTransfer // 13
    transfer_scan_action_item!( act, HacFromTransfer, ck.hacash, json!({
        "from": act.from.readable(),
        "hacash": act.amt.to_unit_string(unit),
    }));
    // HacFromToTransfer // 14
    transfer_scan_action_item!( act, HacFromToTransfer, ck.hacash, json!({
        "from": act.from.readable(),
        "to": act.to.readable(),
        "hacash": act.amt.to_unit_string(unit),
    }));

    // DiamondTransfer          // 5 
    transfer_scan_action_item!( act, DiamondTransfer, ck.diamond, json!({
        "to": act.to.readable(),
        "diamond": 1usize,
        "diamonds": act.diamond.readable(),
    }));
    // DiamondFromToTransfer    // 6
    transfer_scan_action_item!( act, DiamondFromToTransfer, ck.diamond, json!({
        "from": act.from.readable(),
        "to": act.to.readable(),
        "diamond": act.diamonds.count().uint(),
        "diamonds": act.diamonds.readable(),
    }));
    // DiamondMultipleTransfer  // 7
    transfer_scan_action_item!( act, DiamondMultipleTransfer, ck.diamond, json!({
        "to": act.to.readable(),
        "diamond": act.diamonds.count().uint(),
        "diamonds": act.diamonds.readable(),
    }));

    // SatoshiTransfer // 8
    transfer_scan_action_item!( act, SatoshiTransfer, ck.satoshi, json!({
        "to": act.to.readable(),
        "satoshi": act.satoshi.uint(),
    }));
    // SatoshiFromToTransfer // 11
    transfer_scan_action_item!( act, SatoshiFromToTransfer, ck.satoshi, json!({
        "from": act.from.readable(),
        "to": act.to.readable(),
        "satoshi": act.satoshi.uint(),
    }));
    // SatoshiFromTransfer // 28
    transfer_scan_action_item!( act, SatoshiFromTransfer, ck.satoshi, json!({
        "from": act.from.readable(),
        "satoshi": act.satoshi.uint(),
    }));


}
