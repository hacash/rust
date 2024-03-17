
pub fn splice_key(p: &[u8], k: &dyn Serialize) -> Vec<u8> {
    let nk = k.serialize();
    let key = [p.to_vec(), nk].concat();
    key
}
    