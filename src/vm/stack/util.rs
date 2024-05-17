

fn buf_is_not_zero(buf: &[u8]) -> bool {
    if buf.len() == 0 {
        return false // empty is zero
    }
    for a in buf {
        if *a != 0 {
            return true
        }
    }
    false
}