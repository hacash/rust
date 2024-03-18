
pub const _BUF_E1: &str = "buffer too short";


#[macro_export]
macro_rules! er {
    ($v:expr) => { Some(($v).to_string()) };
}

macro_rules! erf {
    ( $($v:expr),+ ) => { er!(format!( $($v),+ )) };
}

#[macro_export]
macro_rules! err {
    ($v:expr) => { Err(($v).to_string()) };
}

macro_rules! errf {
    ( $($v:expr),+ ) => { err!(format!( $($v),+ )) };
}

#[macro_export]
macro_rules! err_buf_short {
    () => { err!(_BUF_E1) };
}