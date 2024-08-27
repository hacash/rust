
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
            "hacash", action.amt.to_unit_string(unit),
        };

    }else if kind == HacFromTransfer::kid() {

        let action = HacFromTransfer::must(&act.serialize());
        let from = action.from.real(adrs).unwrap();
        resjsonobj =  jsondata!{
            "from", from.readable(),
            "to", main_addr_str,
            "hacash", action.amt.to_unit_string(unit),
        };

    }else if kind == HacFromToTransfer::kid() {

        let action = HacFromToTransfer::must(&act.serialize());
        let from = action.from.real(adrs).unwrap();
        let to = action.to.real(adrs).unwrap();
        resjsonobj =  jsondata!{
            "from", from.readable(),
            "to", to.readable(),
            "hacash", action.amt.to_unit_string(unit),
        };
    

    /*************** Hacash ***************/


    }else if kind == SatoshiToTransfer::kid() {

        let action = SatoshiToTransfer::must(&act.serialize());
        let to = action.to.real(adrs).unwrap();
        resjsonobj =  jsondata!{
            "from", main_addr_str,
            "to", to.readable(),
            "satoshi", action.satoshi.uint(),
        };

    }else if kind == SatoshiFromTransfer::kid() {

        let action = SatoshiFromTransfer::must(&act.serialize());
        let from = action.from.real(adrs).unwrap();
        resjsonobj = jsondata!{
            "from", from.readable(),
            "to", main_addr_str,
            "satoshi", action.satoshi.uint(),
        };

    }else if kind == SatoshiFromToTransfer::kid() {

        let action = SatoshiFromToTransfer::must(&act.serialize());
        let from = action.from.real(adrs).unwrap();
        let to = action.to.real(adrs).unwrap();
        resjsonobj = jsondata!{
            "from", from.readable(),
            "to", to.readable(),
            "satoshi", action.satoshi.uint(),
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


    /*************** Diamond mint & inscription ***************/


    }else if kind == DiamondMint::kid() {

        let action = DiamondMint::must(&act.serialize());
        resjsonobj = jsondata!{
            "name", action.head.diamond.readable(),
            "number", action.head.number.uint(),
            "miner", action.head.address.readable(),
            "nonce", action.head.nonce.hex(),
            "prev_hash", action.head.prev_hash.hex(), // prev block hash
            "custom_message", action.custom_message.hex(),
        };

    }else if kind == DiamondInscription::kid() {

        let action = DiamondInscription::must(&act.serialize());
        resjsonobj = jsondata!{
            "diamonds", action.diamonds.readable(),
            "protocol_cost", action.protocol_cost.to_unit_string(unit),
            "engraved_type", action.engraved_type.uint(),
            "engraved_content", action.engraved_content.readable_or_hex(),
        };

    }else if kind == DiamondInscriptionClean::kid() {

        let action = DiamondInscriptionClean::must(&act.serialize());
        resjsonobj = jsondata!{
            "diamonds", action.diamonds.readable(),
            "protocol_cost", action.protocol_cost.to_unit_string(unit),
        };



    /*************** Channel ***************/

    }else if kind == ChannelOpen::kid() {

        let action = ChannelOpen::must(&act.serialize());
        resjsonobj = jsondata!{
            "channel_id", action.channel_id.hex(),
            "left", jsondata!{
                "address", action.left_bill.address.readable(),
                "hacash", action.left_bill.amount.to_unit_string(unit),
            },
            "right", jsondata!{
                "address", action.right_bill.address.readable(),
                "hacash", action.right_bill.amount.to_unit_string(unit),
            },
        };

    }else if kind == ChannelClose::kid() {

        let action = ChannelClose::must(&act.serialize());
        resjsonobj = jsondata!{
            "channel_id", action.channel_id.hex(),
        };


    /*************** Others ***************/

    }else if kind == SubmitHeightLimit::kid() {
        
        let action = SubmitHeightLimit::must(&act.serialize());
        resjsonobj = jsondata!{
            "start_height", action.start.uint(),
            "end_height", action.end.uint(),
        };

    }else if kind == SubChainID::kid() {
        
        let action = SubChainID::must(&act.serialize());
        resjsonobj = jsondata!{
            "chain_id", action.chain_id.uint(),
        };


    }else{

    }

    // ok
    if ret_kind {
        resjsonobj.insert("kind", json!(kind));
    }
    return resjsonobj
}