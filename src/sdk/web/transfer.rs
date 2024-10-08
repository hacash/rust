static mut API_RETURN_JSON: bool = true;




/*
#[no_mangle]
pub extern fn trs_test(x: i32) -> i32 {
    let mut bts = vec![1,0,5,1,1,1,1,1,1,1];
    bts[1] = x;
    if x > 100 {
        panic!("error more 100")
    }
    let mut res = 0;
    for v in bts {
        res += v;
    }
    res + 10
}
*/



#[no_mangle]
pub extern fn trs_test(x: i32) -> usize {
    let mut bt = field_bnk::Fixed4::default();
    let data = vec![x as u8 + 1, x as u8 + 2, x as u8 + 3, x as u8 + 4];
    let mut res = bt.parse(&data, 0).unwrap();
    let vals = bt.serialize();
    res += 1;
    res = res + vals[x as usize] as usize;
    res += x as usize;
    let vvs = bt.hex().into_bytes();
    res += vvs[2] as usize;
    res

    // x as usize + data[0] as usize
    // x as usize + 1
}

use crate::core::account::Account;

#[no_mangle]
pub extern fn create_acc_random() -> usize {
    let acc = Account::create_by_password(&"123456".to_string());
    if let Err(e) = acc {
        return 0
    } 
    let accstr = acc.unwrap().readable().clone();
    let bts = accstr.as_bytes();

    bts[1] as usize

}


//////////////////////////////
#[no_mangle]
pub extern fn set_api_return_json() {
}






fn if_add_chain_id(chain_id: u64, tx: &mut impl Transaction) {
    // act
    if chain_id > 0 {
        let mut act = action::new_CheckChainID();
        act.chain_id = Uint8::from(chain_id);
        tx.append_action(Box::new(act));
    }
}

fn get_time_set(timestamp: i64) -> i64 {
    let mut time_set = timestamp;
    if time_set <= 0 {
        time_set = Utc::now().timestamp();
    }
    time_set
}


#[wasm_bindgen]
pub fn general_transfer(chain_id: u64, from_pass: String, to_addr: String, amountex: String, fee: String, timestamp: i64) -> String {
    let amount = amountex.clone().to_uppercase().replace(" ","");
    // HACD
    let res1 = DiamondListMax200::parse_from_list(amount.clone());
    if let Ok(diamonds) = res1 {
        return hacd_transfer(chain_id, from_pass.clone(), from_pass.clone(), to_addr, amount, fee, timestamp);
    }    
    // SAT
    let res2 = amount.find("SAT"); // SAT, SATS, SATOSHI, SATOSHIS
    if let Some(_) = res2 {
        let v = amount.replace("S","").replace("AT","").replace("OHI","");
        if let Ok(sat) = v.parse::<u64>() {
            return sat_transfer(chain_id, from_pass.clone(), from_pass.clone(), to_addr, sat, fee, timestamp);
        }
    }
    // HAC
    let res3 =  Amount::from_string_unsafe(&amount);
    if let Ok(hac) = res3 {
        return hac_transfer(chain_id, from_pass.clone(), to_addr, amount, fee, timestamp);
    }

    // AMOUNT ERROR
    or_return!{"Amount format", Err(amount)};

    return "[ERROR]".to_string()
}




#[wasm_bindgen]
pub fn hac_transfer(chain_id: u64, from_pass: String, to_addr: String, amount: String, fee: String, timestamp: i64) -> String {
    let time_set = get_time_set(timestamp);
    // amount
    let amt = or_return!{ "Amount parse", Amount::from_string_unsafe(&amount) };
    let fee = or_return!{ "Fee parse", Amount::from_string_unsafe(&fee) };
    let acc = or_return!{ "From Account", Account::create_by(&from_pass) };
    let toaddr = or_return!{ "To Address", Address::from_readable(&to_addr) };
    // tx
    let mut tx = transaction::new_type_2(acc.address(), &fee, time_set);
    // chain id
    if_add_chain_id(chain_id, &mut tx);
    // actions
    let act = action_create!{ HacTransfer,
        to_address: toaddr.clone(),
        amount: amt.clone()
    };
    tx.append_action(Box::new(act));
    // sign
    tx.fill_sign(&acc);

    // ok
    // format!("{},{},{},{}", hex::encode(2u64.to_be_bytes()), hex::encode(Uint1::from_uint(2)), tx.hash().hex(), hex::encode(tx.serialize()))
    // format!("{},{},{},{},{},{},{},{}", tx.hash().hex(), hex::encode(tx.serialize()), chain_id, acc.readable(), toaddr.readable(), amt.to_fin_string(), fee.to_fin_string(), time_set)
    // format!("{},{},{},{},{}", tx.hash().hex(), hex::encode(tx.serialize()), acc.readable(), acc.readable(), time_set)

    let ok = format!(r##""tx_hash":"{}","tx_body":"{}","amount":"{}","fee":"{}","payment_address":"{}","fee_address":"{}","collection_address":"{}","timestamp":{}"##, 
        tx.hash().hex(), hex::encode(tx.serialize()), amt.to_fin_string(), fee.to_fin_string(), acc.readable(), acc.readable(), toaddr.readable(), time_set);
    format!("{{{}}}", ok)
}


#[wasm_bindgen]
pub fn sat_transfer(chain_id: u64, from_pass: String, fee_pass: String, to_addr: String, satoshi: u64, fee: String, timestamp: i64) -> String {
    let time_set = get_time_set(timestamp);
    // amount
    let sat = Satoshi::from_uint(satoshi);
    let fee = or_return!{ "Fee parse", Amount::from_string_unsafe(&fee) };
    let acc = or_return!{ "From Account", Account::create_by(&from_pass) };
    let feeacc = or_return!{ "Fee Account", Account::create_by(&fee_pass) };
    let toaddr = or_return!{ "To Address", Address::from_readable(&to_addr) };
    // tx
    let is_main_single = feeacc.address() == acc.address();
    let mut tx = transaction::new_type_2(feeacc.address(), &fee, time_set);
    // chain id
    if_add_chain_id(chain_id, &mut tx);
    // actions
    if is_main_single {
        let act = action_create!{ SatTransfer,
            to_address: toaddr.clone(),
            satoshi: sat.clone()
        };
        tx.append_action(Box::new(act));
    }else{
        let act = action_create!{ FromToSatTransfer,
            from_address: acc.address().clone(),
            to_address: toaddr.clone(),
            satoshi: sat.clone()
        };
        tx.append_action(Box::new(act));
    }
    // sign
    tx.fill_sign(&acc);
    if !is_main_single {
        tx.fill_sign(&feeacc);
    }

    // ok
    // format!("{},{},{},{}", hex::encode(2u64.to_be_bytes()), hex::encode(Uint1::from_uint(2)), tx.hash().hex(), hex::encode(tx.serialize()))
    // format!("{},{},{},{},{},{},{},{}", tx.hash().hex(), hex::encode(tx.serialize()), chain_id, acc.readable(), toaddr.readable(), amt.to_fin_string(), fee.to_fin_string(), time_set)
    // format!("{},{},{},{},{}", tx.hash().hex(), hex::encode(tx.serialize()), acc.readable(), feeacc.readable(), time_set)

    let ok = format!(r##""tx_hash":"{}","tx_body":"{}","amount":"{} SAT","fee":"{}","payment_address":"{}","fee_address":"{}","collection_address":"{}","timestamp":{}"##, 
        tx.hash().hex(), hex::encode(tx.serialize()), sat.to_u64(), fee.to_fin_string(), acc.readable(), feeacc.readable(), toaddr.readable(), time_set);
    format!("{{{}}}", ok)

}


#[wasm_bindgen]
pub fn hacd_transfer(chain_id: u64, from_pass: String, fee_pass: String, to_addr: String, diamond_name_list: String, fee: String, timestamp: i64) -> String {
    let time_set = get_time_set(timestamp);
    // data
    
    let dlist = or_return!{ "Diamond Name parse", DiamondListMax200::parse_from_list(diamond_name_list) };
    let fee = or_return!{ "Fee parse", Amount::from_string_unsafe(&fee) };
    let acc = or_return!{ "From Account", Account::create_by(&from_pass) };
    let feeacc = or_return!{ "Fee Account", Account::create_by(&fee_pass) };
    let toaddr = or_return!{ "To Address", Address::from_readable(&to_addr) };
    // tx
    let is_main_single = feeacc.address() == acc.address();
    let mut tx = transaction::new_type_2(feeacc.address(), &fee, time_set);
    // chain id
    if_add_chain_id(chain_id, &mut tx);
    // actions
    if is_main_single && dlist.len() == 1 {
        let act = action_create!{ HacdTransfer,
            diamond: dlist[0],
            to_address: toaddr.clone()
        };
        tx.append_action(Box::new(act));
    }else{
        let act = action_create!{ HacdTransferMultiple,
            from_address: acc.address().clone(),
            to_address: toaddr.clone(),
            diamond_list: dlist.clone()
        };
        tx.append_action(Box::new(act));
    }
    // sign
    tx.fill_sign(&acc);
    if !is_main_single {
        tx.fill_sign(&feeacc);
    }
    
    // ok
    // format!("{},{},{},{}", hex::encode(2u64.to_be_bytes()), hex::encode(Uint1::from_uint(2)), tx.hash().hex(), hex::encode(tx.serialize()))
    // format!("{},{},{},{},{},{},{},{}", tx.hash().hex(), hex::encode(tx.serialize()), chain_id, acc.readable(), toaddr.readable(), amt.to_fin_string(), fee.to_fin_string(), time_set)
    // format!("{},{},{},{},{}", tx.hash().hex(), hex::encode(tx.serialize()), acc.readable(), feeacc.readable(), time_set)

    let ok = format!(r##""tx_hash":"{}","tx_body":"{}","diamond_count":{},"diamonds":"{}","fee":"{}","payment_address":"{}","fee_address":"{}","collection_address":"{}","timestamp":{}"##, 
        tx.hash().hex(), hex::encode(tx.serialize()), dlist.len(), dlist.to_string(), fee.to_fin_string(), acc.readable(), feeacc.readable(), toaddr.readable(), time_set);
    format!("{{{}}}", ok)

}



