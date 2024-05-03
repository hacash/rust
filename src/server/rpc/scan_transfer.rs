
defineQueryObject!{ Q4538,
    height, u64, 1,
    txposi, usize, 0,
}

async fn scan_coin_transfer(State(ctx): State<ApiCtx>, q: Query<Q4538>) -> impl IntoResponse {
    ctx_store!(ctx, store);
    let unit = q_unit!(q);
    let kind = q_coinkind!(q);
    // println!("q_coinkind = {:?}", kind);
    let blkpkg = ctx.load_block(&store, q.height);
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
        append_transfer_scan(&unit, &kind, dtlistptr, act.as_ref());
    }
    // ok
    let mut data = jsondata!{
        "tx_hash", tartrs.hash().hex(),
        "block_hash", blkobj.hash().hex(),
        "tx_timestamp", tartrs.timestamp().uint(),
        "block_timestamp", blkobj.timestamp().uint(),
        "address", tartrs.address().readable(),
        "transfers", dtlist
    };
    api_data(data)
}




macro_rules! transfer_scan_action_item{
    ( $kid: expr, $aext: expr, $transfers: expr, $actname: ident, $acty: ty, $ck: expr, $jsonobj: expr ) => (
        if $kid == <$acty>::kid(){
            if false == $ck {
                return
            }
            let ($actname, _) = <$acty>::create(&$aext.serialize()).unwrap();
            $transfers.push($jsonobj);
            return
        }
    )
}


fn append_transfer_scan(unit: &str, ck: &CoinKind, transfers: &mut Vec<Value>, a: &dyn VMAction) {
    let act = a.as_ext();
    let akd = act.kind();
    // HAC transfer
    transfer_scan_action_item!(akd, a, transfers, act, 
        HacTransfer, ck.hacash, json!({
        "to": act.to.readable(),
        "hacash": act.amt.to_unit_string(unit),
    }));
    // HAC from transfer
    transfer_scan_action_item!(akd, a, transfers, act, 
        HacFromTransfer, ck.hacash, json!({
        "from": act.from.readable(),
        "hacash": act.amt.to_unit_string(unit),
    }));
    // HAC from to transfer
    transfer_scan_action_item!(akd, a, transfers, act, 
        HacFromToTransfer, ck.hacash, json!({
        "from": act.from.readable(),
        "to": act.to.readable(),
        "hacash": act.amt.to_unit_string(unit),
    }));


}
