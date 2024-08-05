
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
    let to = Address::form_readable(&q.to_address);
    if let Err(e) = to {
        return api_error(&format!("to address {} format error: {}", &q.to_address, &e))
    }
    let toaddr = to.unwrap();
    let fee = Amount::from_string_unsafe(&q.fee);
    if let Err(e) = fee {
        return api_error(&format!("fee {} format error: {}", &q.fee, &e))
    }
    let acc = account::Account::create_by(&q.main_prikey);
    if let Err(e) = acc {
        return api_error(&format!("main prikey error: {}", &e))
    }
    let main_acc = acc.unwrap();
    let mut from_acc = main_acc.clone();
    if from_prikey.len() > 0 {
        let fc = account::Account::create_by(&from_prikey);
        if let Err(e) = fc {
            return api_error(&format!("from prikey error: {}", &e))
        }
        from_acc = fc.unwrap();
    }
    let is_from = from_acc != main_acc;
    let addr = Address::cons(main_acc.address().clone());
    let fromaddr = Address::cons(from_acc.address().clone());
    // trs v2
    let mut trsobj = TransactionType2::build(addr, fee.unwrap());
    if timestamp > 0 {
        trsobj.timestamp = Timestamp::from(timestamp);
    }
    // append actions
    // sat
    if satoshi > 0 {
        let mut act: Box<dyn Action>;
        let sat = Satoshi::from(satoshi);
        if is_from {
            let mut obj = SatoshiFromToTransfer::default();
            obj.from = AddrOrPtr::by_addr(fromaddr);
            obj.to = AddrOrPtr::by_addr(toaddr);
            obj.satoshi = sat;
            act = Box::new(obj);
        }else{
            let mut obj = SatoshiTransfer::default();
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
            let mut obj = DiamondFromToTransfer::default();
            obj.from = AddrOrPtr::by_addr(fromaddr);
            obj.to = AddrOrPtr::by_addr(toaddr);
            obj.diamonds = dialist;
            act = Box::new(obj);
        }else{
            if dialist.count().uint() == 1 {
                let mut obj = DiamondTransfer::default();
                obj.to = AddrOrPtr::by_addr(toaddr);
                obj.diamond = DiamondName::cons(*dialist.list()[0]);
                act = Box::new(obj);
            }else{
                let mut obj = DiamondMultipleTransfer::default();
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
            let mut obj = HacFromToTransfer::default();
            obj.from = AddrOrPtr::by_addr(fromaddr);
            obj.to = AddrOrPtr::by_addr(toaddr);
            obj.amt = hac;
            act = Box::new(obj);
        }else{
            let mut obj = HacTransfer::default();
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
    // ok ret
    let mut data = jsondata!{
        "hash", trsobj.hash().hex(),
        "hash_with_fee", trsobj.hash_with_fee().hex(),
        "timestamp", trsobj.timestamp().uint(),
        "body", hex::encode(trsobj.serialize()),
    };
    api_data(data)
}


