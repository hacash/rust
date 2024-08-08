
defineQueryObject!{ Q9374,
    fee, String, s!(""),
    main_prikey, String, s!(""),
    to_address, String, s!(""),
    timestamp, Option<u64>, None,
    from_prikey, Option<String>, None,
    // asset
    hacash, Option<String>, None,
    satoshi, Option<u64>, None,
    diamonds, Option<String>, None,
}

async fn create_coin_transfer(State(ctx): State<ApiCtx>, q: Query<Q9374>) -> impl IntoResponse {
    ctx_state!(ctx, state);
    q_must!(q, from_prikey, s!(""));
    q_must!(q, timestamp, 0);
    // q_unit!(q);
    q_must!(q, satoshi, 0);
    q_must!(q, hacash, s!(""));
    q_must!(q, diamonds, s!(""));
    // create
    let toaddr = q_data_addr!(q, to_address);
    let fee = q_data_amt!(q, fee);
    let main_acc = q_data_acc!(q, main_prikey);

    let mut from_acc = main_acc.clone();
    if from_prikey.len() > 0 {
        from_acc = q_data_acc_from!(from_prikey);
    }
    let is_from = from_acc != main_acc;
    let addr = Address::cons(main_acc.address().clone());
    let fromaddr = Address::cons(from_acc.address().clone());
    // trs v2
    let mut trsobj = TransactionType2::build(addr, fee);
    if timestamp > 0 {
        trsobj.timestamp = Timestamp::from(timestamp);
    }
    // append actions
    // sat
    if satoshi > 0 {
        let mut act: Box<dyn Action>;
        let sat = Satoshi::from(satoshi);
        if is_from {
            let mut obj = SatoshiFromToTransfer::new();
            obj.from = AddrOrPtr::by_addr(fromaddr);
            obj.to = AddrOrPtr::by_addr(toaddr);
            obj.satoshi = sat;
            act = Box::new(obj);
        }else{
            let mut obj = SatoshiTransfer::new();
            obj.to = AddrOrPtr::by_addr(toaddr);
            obj.satoshi = sat;
            act = Box::new(obj);
        }
        trsobj.push_action(act);
    }
    // hacd
    if diamonds.len() >= DiamondName::width() {
        let mut act: Box<dyn Action>;
        let dialist = DiamondNameListMax200::from_string(&diamonds);
        if let Err(e) = dialist {
            return api_error(&format!("diamonds error: {}", &e))
        }
        let dialist = dialist.unwrap();
        if is_from {
            let mut obj = DiamondFromToTransfer::new();
            obj.from = AddrOrPtr::by_addr(fromaddr);
            obj.to = AddrOrPtr::by_addr(toaddr);
            obj.diamonds = dialist;
            act = Box::new(obj);
        }else{
            if dialist.count().uint() == 1 {
                let mut obj = DiamondTransfer::new();
                obj.to = AddrOrPtr::by_addr(toaddr);
                obj.diamond = DiamondName::cons(*dialist.list()[0]);
                act = Box::new(obj);
            }else{
                let mut obj = DiamondMultipleTransfer::new();
                obj.to = AddrOrPtr::by_addr(toaddr);
                obj.diamonds = dialist;
                act = Box::new(obj);
            }
        }
        trsobj.push_action(act);
    }
    // hac
    if hacash.len() > 0 {
        let mut act: Box<dyn Action>;
        let hac = Amount::from_string_unsafe(&hacash);
        if let Err(e) = hac {
            return api_error(&format!("hacash amount {} error: {}", &hacash, &e))
        }
        let hac = hac.unwrap();
        if is_from {
            let mut obj = HacFromToTransfer::new();
            obj.from = AddrOrPtr::by_addr(fromaddr);
            obj.to = AddrOrPtr::by_addr(toaddr);
            obj.amt = hac;
            act = Box::new(obj);
        }else{
            let mut obj = HacTransfer::new();
            obj.to = AddrOrPtr::by_addr(toaddr);
            obj.amt = hac;
            act = Box::new(obj);
        }
        trsobj.push_action(act);
    }
    // do sign
    if let Err(e) = trsobj.fill_sign(&main_acc) {
        return api_error(&format!("fill main sgin error: {}", e))
    }
    if is_from {
        if let Err(e) = trsobj.fill_sign(&from_acc) {
            return api_error(&format!("fill from sgin error: {}", e))
        }
    }
    if let Err(e) = trsobj.verify_signature() {
        return api_error(&format!("verify signature error: {}", e))
    }
    // ok ret
    let mut data = jsondata!{
        "hash", trsobj.hash().hex(),
        "hash_with_fee", trsobj.hash_with_fee().hex(),
        "timestamp", trsobj.timestamp().uint(),
        "body", trsobj.serialize().hex(),
    };
    api_data(data)
}


