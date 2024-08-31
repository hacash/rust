
#[wasm_bindgen]
pub fn sign(acckey: String, msg: String) -> String {
    let acc = or_return!{ "create account", Account::create_by(&acckey) };
    // acc ok
    // let acckey = hex::encode(acc.secret_key().serialize());
    let accpub = acc.public_key().serialize_compressed();
    // sign
    let msghx = or_return!{ "parse message", hex::decode(&msg) };
    if msghx.len() != 32 {
        return "[ERROR] message must size 32".to_string()
    }
    let msgbts: &[u8; 32] = msghx[..].try_into().unwrap();
    let sigdts = acc.do_sign(msgbts);
    // ok
    let res = format!(r##""address":"{}","message":"{}","pubkey":"{}","sigdts":"{}""##, 
        acc.readable(), msg, hex::encode(accpub), hex::encode(sigdts));
    format!("{{{}}}", res)
}


