
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
    let acc = Account::create_by_password("123456".to_string());
    if let Err(e) = acc {
        return 0
    } 
    let accstr = acc.unwrap().readable().clone();
    let bts = accstr.as_bytes();

    bts[1] as usize

}


#[wasm_bindgen]
pub fn create_account_by_string(s: String) -> String {
    let acc = Account::create_by_password(s);
    if let Err(e) = acc {
        return e.to_string()
    } 
    let acc = acc.unwrap();
    let accstr = acc.readable();
    let acckey = hex::encode(acc.secret_key().serialize());
    let accpub = hex::encode(acc.public_key().serialize());
    format!("{},{},{}", acckey, accpub, accstr)
}

