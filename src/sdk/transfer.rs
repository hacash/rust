
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
    let mut bt = field::Fixed4::new();
    let data = vec![x as u8 + 1, x as u8 + 2, x as u8 + 3, x as u8 + 4];
    let mut res = bt.parse(&data, 0).unwrap();
    let vals = bt.serialize();
    res += 1;
    res = res + vals[x as usize] as usize;
    res += x as usize;
    let vvs = bt.to_hex().into_bytes();
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


#[wasm_bindgen]
pub fn create_account_by(s: String) -> String {
    let acc = Account::create_by(&s);
    if let Err(e) = acc {
        return e.to_string()
    } 
    let acc = acc.unwrap();
    let accstr = acc.readable();
    let acckey = hex::encode(acc.secret_key().serialize());
    let accpub = hex::encode(acc.public_key().serialize_compressed());
    format!("{},{},{}", acckey, accpub, accstr)
}


macro_rules! or_return {
    ($tip:expr, $gain:expr) => (
        match $gain {
            Ok(obj) => obj,
            Err(e) => {
                return format!("[ERROR] {}: {}", $tip, e)
            }
        }

    )
}

#[wasm_bindgen]
pub fn hac_transfer(chain_id: u64, from_pass: String, to_addr: String, amount: String, fee: String, timestamp: i64) -> String {
    let mut time_set = timestamp;
    if time_set <= 0 {
        time_set = Utc::now().timestamp();
    }
    // amount
    let amt = or_return!{ "Amount parse", Amount::from_string_unsafe(&amount) };
    let fee = or_return!{ "Fee parse", Amount::from_string_unsafe(&fee) };
    let acc = or_return!{ "From Account", Account::create_by(&from_pass) };
    let toaddr = or_return!{ "To Address", Address::form_readable(&to_addr) };
    // tx
    let mut tx = transaction::new_type_2(acc.address(), &fee, time_set);
    let mut act = action::new_HacTransfer();
    act.to_address = toaddr.clone();
    act.amount = amt.clone();
    tx.append_action(Box::new(act));
    // act
    if chain_id > 0 {
        let mut act = action::new_CheckChainID();
        act.chain_id = Uint8::from_uint(chain_id);
        tx.append_action(Box::new(act));
    }
    tx.fill_sign(&acc);

    // ok
    // format!("{},{},{},{}", hex::encode(2u64.to_be_bytes()), hex::encode(Uint1::from_uint(2)), tx.hash().to_hex(), hex::encode(tx.serialize()))
    // format!("{},{},{},{},{},{},{},{}", tx.hash().to_hex(), hex::encode(tx.serialize()), chain_id, acc.readable(), toaddr.to_readable(), amt.to_fin_string(), fee.to_fin_string(), time_set)
    format!("{},{},{},{}", tx.hash().to_hex(), hex::encode(tx.serialize()), acc.readable(), time_set)
}