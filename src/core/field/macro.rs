
// parse move seek check
macro_rules! parse_move_seek_or_buf_too_short_error{
    ($tip:expr, $seek:expr, $sk:expr, $buf:expr) => ( {
        let mvseek = $seek + $sk;
        let buflen = $buf.len();
        match mvseek <= buflen {
            true => mvseek,
            false => {
                let n1 = &mvseek.to_string();
                let n2 = &buflen.to_string();
                return Err("field::".to_owned()+$tip+".parse() buf too short, need "+n1+" but got "+n2)
            },
        }
    })
}

