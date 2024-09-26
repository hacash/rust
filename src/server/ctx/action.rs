
// json string
pub fn action_from_json(main_addr: &Address, jsonv: &serde_json::Value) -> Ret<Box<dyn Action>> {
    let _ = jsonv.as_object() else {
        return errf!("action format error")
    };

    let Some(kind) = jsonv["kind"].as_u64() else {
        return errf!("kind format error")
    };
    if kind > u16::MAX.into(){
        return errf!("kind {} value overflow", kind)
    }
    let kind = kind as u16;

    macro_rules! j_addr {
        ($k: expr) => ({
            let Some(adr) = jsonv[$k].as_str() else {
                return errf!("address format error")
            };
            let Ok(adrobj) = Address::from_readable(adr) else {
                return errf!("address {} error", adr)
            };
            AddrOrPtr::from_addr(adrobj)
        })
    }

    macro_rules! j_hac { // hac
        ($k: expr) => ({
            let Some(amt) = jsonv[$k].as_str() else {
                return errf!("amount format error")
            };
            let Ok(amtobj) = Amount::from_string_unsafe(amt) else {
                return errf!("amount {} error", amt)
            };
            amtobj
        })
    }

    macro_rules! j_sat { // satoshi
        ($k: expr) => ({
            let Some(sat) = jsonv[$k].as_u64() else {
                return errf!("satoshi format error")
            };
            Satoshi::from(sat)
        })
    }

    macro_rules! j_dias { // diamonds
        ($k: expr) => ({
            let Some(dias) = jsonv[$k].as_str() else {
                return errf!("diamonds format error")
            };
            let dialist = DiamondNameListMax200::from_readable(dias);
            if let Err(e) = dialist {
                return errf!("diamonds {} error",  &e)
            }
            dialist.unwrap()
        })
    }

    macro_rules! j_uint {
        ($k: expr, $t1: ty, $t2: ty) => ({
            let Some(num) = jsonv[$k].as_u64() else {
                return errf!("{} format error", stringify!($k))
            };
            if num > <$t1>::MAX.into() {
                return errf!("{} value overflow", stringify!($k))
            }
            <$t2>::from(num as $t1)
        })
    }

    macro_rules! j_uint1 {
        ($k: expr) => (
            j_uint!($k, u8, Uint1)
        )
    }

    macro_rules! j_uint5 {
        ($k: expr) => (
            j_uint!($k, u64, Uint5)
        )
    }

    macro_rules! j_uint8 {
        ($k: expr) => (
            j_uint!($k, u64, Uint8)
        )
    }

    macro_rules! j_bytes {
        ($k: expr, $t1: ty, $t2: ty) => ({
            let Some(btstr) = jsonv[$k].as_str() else {
                return errf!("{} format error", stringify!($k))
            };
            let bts = match hex::decode(btstr) {
                Ok(b) => b,
                _ => btstr.as_bytes().to_vec(),
            };
            if bts.len() > <$t1>::MAX.into() {
                return errf!("{} length overflow", stringify!($k))
            }
            <$t2>::from_vec(bts)
        })
    }

    macro_rules! j_bytes1 {
        ($k: expr) => (
            j_bytes!($k, u8, BytesW1)
        )
    }

    macro_rules! ret_act {
        ( $cls: ident, $( $k: ident, $v: expr)+ ) => {
            Ok(Box::new($cls {
                kind: Uint2::from(<$cls>::kid()),
                $(
                    $k: $v,
                )+
            }))
        }
    }

    macro_rules! if_ret_act {
        ( $cls: ident, $( $k: ident, $v: expr)+ ) => {
            if kind == <$cls>::kid() {
                return ret_act!{ $cls, 
                    $(
                        $k, $v
                    )+
                }
            }
        }
    }
    macro_rules! if_ret_act_ns {
        ( $cls: ident, $( $k: ident, $g: ident)+ ) => {
            if kind == <$cls>::kid() {
                return ret_act!{ $cls, 
                    $(
                        $k, $g!( stringify!($k) )
                    )+
                }
            }
        }
    }

    /*********** Hacash ***********/

    if_ret_act_ns!{ HacToTransfer,
        to,       j_addr
        hacash,   j_hac
    }

    if_ret_act_ns!{ HacFromTransfer, 
        from,     j_addr
        hacash,   j_hac
    }

    if_ret_act_ns!{ HacFromToTransfer, 
        from,     j_addr
        to,       j_addr
        hacash,   j_hac
    }

    /*********** Satoshi ***********/

    if_ret_act_ns!{ SatoshiToTransfer,
        to,       j_addr
        satoshi,  j_sat
    }

    if_ret_act_ns!{ SatoshiFromTransfer, 
        from,     j_addr
        satoshi,  j_sat
    }

    if_ret_act_ns!{ SatoshiFromToTransfer, 
        from,     j_addr
        to,       j_addr
        satoshi,  j_sat
    }

    /*********** Diamond ***********/

    if_ret_act!{ DiamondSingleTransfer,
        to,       j_addr!("to")
        diamond,  j_dias!("diamonds")[0]
    }

    if_ret_act_ns!{ DiamondToTransfer,
        to,       j_addr
        diamonds, j_dias
    }

    if_ret_act_ns!{ DiamondFromTransfer, 
        from,     j_addr
        diamonds, j_dias
    }

    if_ret_act_ns!{ DiamondFromToTransfer, 
        from,     j_addr
        to,       j_addr
        diamonds, j_dias
    }

    if_ret_act_ns!{ DiamondInscription,
        diamonds,         j_dias
        protocol_cost,    j_hac
        engraved_type,    j_uint1
        engraved_content, j_bytes1
    }

    if_ret_act_ns!{ DiamondInscriptionClear,
        diamonds,         j_dias
        protocol_cost,    j_hac
    }


    /*********** Other ***********/


    if_ret_act_ns!{ SubmitHeightLimit,
        start,         j_uint5
        end,           j_uint5
    }

    if_ret_act_ns!{ SubChainID,
        chain_id,      j_uint8
    }





    // not support
    return errf!("kind {} not support", kind)
}



// json string
pub fn action_to_json_desc(tx: &dyn TransactionRead, act: &dyn Action, 
    unit: &String, ret_kind: bool, ret_desc: bool
) -> JsonObject {

    let adrs = tx.addrlist();
    let main_addr = tx.address().unwrap().readable();
    let kind = act.kind();

    let mut resjsonobj = jsondata!{
        "kind", kind,
    };



    /*************** Hacash ***************/


    if kind == HacToTransfer::kid() {

        let action = HacToTransfer::must(&act.serialize());
        let to_addr = action.to.real(adrs).unwrap().readable();
        let amt_str = action.hacash.to_unit_string(unit);
        resjsonobj = jsondata!{
            "from", main_addr,
            "to", to_addr,
            "hacash", amt_str,
        };
        if ret_desc {
            resjsonobj.insert("description", json!(format!(
                "Transfer {} HAC from {} to {}",
                amt_str, main_addr, to_addr
            )));
        }

    }else if kind == HacFromTransfer::kid() {

        let action = HacFromTransfer::must(&act.serialize());
        let from_addr = action.from.real(adrs).unwrap().readable();
        let amt_str = action.hacash.to_unit_string(unit);
        resjsonobj = jsondata!{
            "from", from_addr,
            "to", main_addr,
            "hacash", amt_str,
        };
        if ret_desc {
            resjsonobj.insert("description", json!(format!(
                "Transfer {} HAC from {} to {}",
                amt_str, from_addr, main_addr
            )));
        }

    }else if kind == HacFromToTransfer::kid() {

        let action = HacFromToTransfer::must(&act.serialize());
        let from_addr = action.from.real(adrs).unwrap().readable();
        let to_addr = action.to.real(adrs).unwrap().readable();
        let amt_str = action.hacash.to_unit_string(unit);
        resjsonobj = jsondata!{
            "from", from_addr,
            "to", to_addr,
            "hacash", amt_str,
        };
        if ret_desc {
            resjsonobj.insert("description", json!(format!(
                "Transfer {} HAC from {} to {}",
                amt_str, from_addr, to_addr
            )));
        }
    

    /*************** Hacash ***************/


    }else if kind == SatoshiToTransfer::kid() {

        let action = SatoshiToTransfer::must(&act.serialize());
        let to_addr = action.to.real(adrs).unwrap().readable();
        let amt_str = action.satoshi.uint();
        resjsonobj = jsondata!{
            "from", main_addr,
            "to", to_addr,
            "satoshi", amt_str,
        };
        if ret_desc {
            resjsonobj.insert("description", json!(format!(
                "Transfer {} SAT from {} to {}",
                amt_str, main_addr, to_addr
            )));
        }

    }else if kind == SatoshiFromTransfer::kid() {

        let action = SatoshiFromTransfer::must(&act.serialize());
        let from_addr = action.from.real(adrs).unwrap().readable();
        let amt_str = action.satoshi.uint();
        resjsonobj = jsondata!{
            "from", from_addr,
            "to", main_addr,
            "satoshi", amt_str,
        };
        if ret_desc {
            resjsonobj.insert("description", json!(format!(
                "Transfer {} SAT from {} to {}",
                amt_str, from_addr, main_addr
            )));
        }

    }else if kind == SatoshiFromToTransfer::kid() {

        let action = SatoshiFromToTransfer::must(&act.serialize());
        let from_addr = action.from.real(adrs).unwrap().readable();
        let to_addr = action.to.real(adrs).unwrap().readable();
        let amt_str = action.satoshi.uint();
        resjsonobj = jsondata!{
            "from", from_addr,
            "to", to_addr,
            "satoshi", amt_str,
        };
        if ret_desc {
            resjsonobj.insert("description", json!(format!(
                "Transfer {} SAT from {} to {}",
                amt_str, from_addr, to_addr
            )));
        }
    

    /*************** Diamonds ***************/


    }else if kind == DiamondSingleTransfer::kid() {

        let action = DiamondSingleTransfer::must(&act.serialize());
        let to_addr = action.to.real(adrs).unwrap().readable();
        let dia_num = 1u32;
        let dia_names = action.diamond.readable();
        resjsonobj =  jsondata!{
            "from", main_addr,
            "to", to_addr,
            "diamond", dia_num,
            "diamonds", dia_names,
        };
        if ret_desc {
            resjsonobj.insert("description", json!(format!(
                "Transfer {} HACD ({}) from {} to {}",
                dia_num, dia_names, main_addr, to_addr
            )));
        }

    }else if kind == DiamondToTransfer::kid() {

        let action = DiamondToTransfer::must(&act.serialize());
        let to_addr = action.to.real(adrs).unwrap().readable();
        let dia_num = action.diamonds.count().uint();
        let dia_names = action.diamonds.readable();
        resjsonobj =  jsondata!{
            "from", main_addr,
            "to", to_addr,
            "diamond", dia_num,
            "diamonds", dia_names,
        };
        if ret_desc {
            resjsonobj.insert("description", json!(format!(
                "Transfer {} HACD ({}) from {} to {}",
                dia_num, action.diamonds.splitstr(), main_addr, to_addr
            )));
        }

    }else if kind == DiamondFromTransfer::kid() {
        
        let action = DiamondFromTransfer::must(&act.serialize());
        let from_addr = action.from.real(adrs).unwrap().readable();
        let dia_num = action.diamonds.count().uint();
        let dia_names = action.diamonds.readable();
        resjsonobj = jsondata!{
            "from", from_addr,
            "to", main_addr,
            "diamond", dia_num,
            "diamonds", dia_names,
        };
        if ret_desc {
            resjsonobj.insert("description", json!(format!(
                "Transfer {} HACD ({}) from {} to {}",
                dia_num, action.diamonds.splitstr(), from_addr, main_addr
            )));
        }

    }else if kind == DiamondFromToTransfer::kid() {

        let action = DiamondFromToTransfer::must(&act.serialize());
        let from_addr = action.from.real(adrs).unwrap().readable();
        let to_addr = action.to.real(adrs).unwrap().readable();
        let dia_num = action.diamonds.count().uint();
        let dia_names = action.diamonds.readable();
        resjsonobj = jsondata!{
            "from", from_addr,
            "to", to_addr,
            "diamond", dia_num,
            "diamonds", dia_names,
        };
        if ret_desc {
            resjsonobj.insert("description", json!(format!(
                "Transfer {} HACD ({}) from {} to {}",
                dia_num, action.diamonds.splitstr(), from_addr, to_addr
            )));
        }


    /*************** Diamond mint & inscription ***************/


    }else if kind == DiamondMint::kid() {

        let action = DiamondMint::must(&act.serialize());
        let name = action.head.diamond.readable();
        let miner = action.head.address.readable();
        resjsonobj = jsondata!{
            "name", name,
            "number", action.head.number.uint(),
            "miner", miner,
            "nonce", action.head.nonce.hex(),
            "prev_hash", action.head.prev_hash.hex(), // prev block hash
            "custom_message", action.custom_message.hex(),
        };
        if ret_desc {
            resjsonobj.insert("description", json!(format!(
                "Mint HACD ({}) to {}",
                name, miner
            )));
        }

    }else if kind == DiamondInscription::kid() {

        let action = DiamondInscription::must(&act.serialize());
        let dia_num = action.diamonds.count().uint();
        let dia_names = action.diamonds.readable();
        let cost_str = action.protocol_cost.to_unit_string(unit);
        let ins_str = action.engraved_content.readable_or_hex();
        resjsonobj = jsondata!{
            "diamond", dia_num,
            "diamonds", dia_names,
            "protocol_cost", cost_str,
            "engraved_type", action.engraved_type.uint(),
            "engraved_content", ins_str,
        };
        if ret_desc {
            let mut desc = format!("Inscript {} HACD ({}) with \"{}\"",
                dia_num, action.diamonds.splitstr(), ins_str);
            if action.protocol_cost.is_positive() {
                desc += &format!("  cost {} HAC fee", cost_str);
            }
            resjsonobj.insert("description", json!(desc));
        }

    }else if kind == DiamondInscriptionClear::kid() {

        let action = DiamondInscriptionClear::must(&act.serialize());
        let dia_num = action.diamonds.count().uint();
        let dia_names = action.diamonds.readable();
        let cost_str = action.protocol_cost.to_unit_string(unit);
        resjsonobj = jsondata!{
            "diamond", dia_num,
            "diamonds", dia_names,
            "protocol_cost", cost_str,
        };
        if ret_desc {
            resjsonobj.insert("description", json!(format!(
                "Clean inscript {} HACD ({}) cost {} HAC fee",
                dia_num, action.diamonds.splitstr(), cost_str
            )));
        }



    /*************** Channel ***************/

    }else if kind == ChannelOpen::kid() {

        let action = ChannelOpen::must(&act.serialize());
        let cid = action.channel_id.hex();
        let l_adr = action.left_bill.address.readable();
        let l_amt = action.left_bill.amount.to_unit_string(unit);
        let r_adr = action.right_bill.address.readable();
        let r_amt = action.right_bill.amount.to_unit_string(unit);
        resjsonobj = jsondata!{
            "channel_id", cid,
            "left", jsondata!{
                "address", l_adr,
                "hacash", l_amt,
            },
            "right", jsondata!{
                "address", r_adr,
                "hacash", r_amt,
            },
        };
        if ret_desc {
            resjsonobj.insert("description", json!(format!(
                "Open channel {} with left {}: {}, right {}: {}",
                cid, l_adr, l_amt, r_adr, r_amt
            )));
        }


    }else if kind == ChannelClose::kid() {

        let action = ChannelClose::must(&act.serialize());
        let cid = action.channel_id.hex();
        resjsonobj = jsondata!{
            "channel_id", cid,
        };
        if ret_desc {
            resjsonobj.insert("description", json!(format!(
                "Close channel {}",
                cid
            )));
        }


    /*************** Others ***************/

    }else if kind == SubmitHeightLimit::kid() {
        
        let action = SubmitHeightLimit::must(&act.serialize());
        let s_hei = action.start.uint();
        let e_hei = action.end.uint();
        resjsonobj = jsondata!{
            "start_height", s_hei,
            "end_height", e_hei,
        };
        if ret_desc {
            let e_hei = match e_hei == 0 { 
                true=>"Unlimited".to_owned(), false=>e_hei.to_string(),
            };
            resjsonobj.insert("description", json!(format!(
                "Limit height range ({}, {}) ",
                s_hei, e_hei
            )));
        }

    }else if kind == SubChainID::kid() {
        
        let action = SubChainID::must(&act.serialize());
        let cid = action.chain_id.uint();
        resjsonobj = jsondata!{
            "chain_id", cid,
        };
        if ret_desc {
            resjsonobj.insert("description", json!(format!(
                "Valid chain ID {}",
                cid
            )));
        }

    }else{

    }

    // ok
    if ret_kind {
        resjsonobj.insert("kind", json!(kind));
    }
    return resjsonobj
}