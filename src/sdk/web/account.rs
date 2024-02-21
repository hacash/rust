
#[wasm_bindgen]
pub fn create_account_by(s: String) -> String {
    let acc = or_return!{ "create account", Account::create_by(&s) };
    // ok
    let accstr = acc.readable();
    let acckey = hex::encode(acc.secret_key().serialize());
    let accpub = hex::encode(acc.public_key().serialize_compressed());
    // format!("{},{},{}", acckey, accpub, accstr)
    let ok = format!(r##""private_key":"{}","public_key":"{}","address":"{}""##, acckey, accpub, accstr);
    format!("{{{}}}", ok)
}
