
// json string
pub fn action_json_desc(tx: &dyn TransactionRead, act: &dyn Action, unit: &String, ret_kind: bool) -> JsonObject {

    let adrs = tx.addrlist();
    let main_addr_str = tx.address().unwrap().readable();
    let kind = act.kind();

    let mut resjsonobj = jsondata!{
        "kind", kind,
    };



    /*************** Hacash ***************/


    if kind == HacToTransfer::kid() {

        let action = HacToTransfer::must(&act.serialize());
        let to = action.to.real(adrs).unwrap();
        resjsonobj =  jsondata!{
            "from", main_addr_str,
            "to", to.readable(),
            "amount", action.amt.to_unit_string(unit),
        };

    }else if kind == HacFromTransfer::kid() {

        let action = HacFromTransfer::must(&act.serialize());
        let from = action.from.real(adrs).unwrap();
        resjsonobj =  jsondata!{
            "from", from.readable(),
            "to", main_addr_str,
            "amount", action.amt.to_unit_string(unit),
        };

    }else if kind == HacFromToTransfer::kid() {

        let action = HacFromToTransfer::must(&act.serialize());
        let from = action.from.real(adrs).unwrap();
        let to = action.to.real(adrs).unwrap();
        resjsonobj =  jsondata!{
            "from", from.readable(),
            "to", to.readable(),
            "amount", action.amt.to_unit_string(unit),
        };
    

    /*************** Hacash ***************/


    }else if kind == SatoshiToTransfer::kid() {

        let action = SatoshiToTransfer::must(&act.serialize());
        let to = action.to.real(adrs).unwrap();
        resjsonobj =  jsondata!{
            "from", main_addr_str,
            "to", to.readable(),
            "amount", action.satoshi.uint(),
        };

    }else if kind == SatoshiFromTransfer::kid() {

        let action = SatoshiFromTransfer::must(&act.serialize());
        let from = action.from.real(adrs).unwrap();
        resjsonobj = jsondata!{
            "from", from.readable(),
            "to", main_addr_str,
            "amount", action.satoshi.uint(),
        };

    }else if kind == SatoshiFromToTransfer::kid() {

        let action = SatoshiFromToTransfer::must(&act.serialize());
        let from = action.from.real(adrs).unwrap();
        let to = action.to.real(adrs).unwrap();
        resjsonobj = jsondata!{
            "from", from.readable(),
            "to", to.readable(),
            "amount", action.satoshi.uint(),
        };
    

    /*************** Diamonds ***************/


    }else if kind == DiamondSingleTransfer::kid() {

        let action = DiamondSingleTransfer::must(&act.serialize());
        let to = action.to.real(adrs).unwrap();
        resjsonobj =  jsondata!{
            "from", main_addr_str,
            "to", to.readable(),
            "diamond", 1,
            "diamonds", action.diamond.readable(),
        };

    }else if kind == DiamondToTransfer::kid() {

        let action = DiamondToTransfer::must(&act.serialize());
        let to = action.to.real(adrs).unwrap();
        resjsonobj =  jsondata!{
            "from", main_addr_str,
            "to", to.readable(),
            "diamond", action.diamonds.count().uint(),
            "diamonds", action.diamonds.readable(),
        };

    }else if kind == DiamondFromTransfer::kid() {
        
        let action = DiamondFromTransfer::must(&act.serialize());
        let from = action.from.real(adrs).unwrap();
        resjsonobj = jsondata!{
            "from", from.readable(),
            "to", main_addr_str,
            "diamond", action.diamonds.count().uint(),
            "diamonds", action.diamonds.readable(),
        };

    }else if kind == DiamondFromToTransfer::kid() {

        let action = DiamondFromToTransfer::must(&act.serialize());
        let from = action.from.real(adrs).unwrap();
        let to = action.to.real(adrs).unwrap();
        resjsonobj = jsondata!{
            "from", from.readable(),
            "to", to.readable(),
            "diamond", action.diamonds.count().uint(),
            "diamonds", action.diamonds.readable(),
        };

        

    }else{

    }

    // ok
    if ret_kind {
        resjsonobj.insert("kind", json!(kind));
    }
    return resjsonobj
}