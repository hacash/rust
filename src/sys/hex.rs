
pub fn bytes_from_hex(stuff: &[u8], len: usize) -> Result<Vec<u8>, Error> {
    let rsz = stuff.len();
    let tsz = len * 2;
    if rsz < tsz {
        return Err("FieldHex".to_owned()+" from_hex size error need "+&rsz.to_string()+" but got "+&rsz.to_string())
    }
    let bts = hex::decode(stuff);
    match bts {
        Ok(b) => Ok(b[0..len].to_vec()),
        Err(e) => Err(e.to_string()),
    }
}