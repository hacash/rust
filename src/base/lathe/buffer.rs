


#[macro_export]
macro_rules! buf_clip_mvsk {
    ($buf:expr, $len:expr) => ( {
        let buflen = $buf.len();
        match $len > buflen {
            false => $buf[..$len].to_vec(), // clone
            true => {
                let n1 = $len.to_string();
                let n2 = &buflen.to_string();
                return Err("buf len too short need ".to_owned()+&n1+" but got "+&n2)
            },
        }
    })
}

