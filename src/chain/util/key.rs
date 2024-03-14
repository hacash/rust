
pub fn splice_key(p: u8, k: &impl Serialize) -> Vec<u8> {
    let nk = k.serialize();
    let key = [[p].to_vec(), nk].concat();
    key
}
    