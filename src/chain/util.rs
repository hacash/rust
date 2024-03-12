
fn splice_key(p: &str, k: &impl Serialize) -> Vec<u8> {
    let nk = k.serialize();
    let key = [p.as_bytes().to_vec(), nk].concat();
    key
}
    