
// parse move seek check


#[macro_export]
macro_rules! parse_move_seek_or_error {
    ($tip:expr, $seek:expr, $sk:expr, $buf:expr) => ( {
        let mvseek = $seek + $sk;
        let buflen = $buf.len();
        match mvseek <= buflen {
            true => mvseek,
            false => {
                let n1 = &mvseek.to_string();
                let n2 = &buflen.to_string();
                return Err($tip.to_owned()+".parse() buf too short, need "+n1+" but got "+n2)
            },
        }
    })
}


#[macro_export] 
macro_rules! create_field_or_error {
    ($tip:expr, $type:ty, $buf:expr, $seek:expr) => ({
        let res = <$type>::create($buf, $seek);
        match res {
            Err(e) => return Err(format!("{}.create error: {}", $tip, e)),
            Ok(res) => res,
        }
    })
}


