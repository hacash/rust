
macro_rules! transactionsTypeDefineCreateFunc{
    ( $( $trstype:ident, $typev:expr, $class:ty ),+ ) => (

// kind define
$(
    pub const $trstype: u8 = $typev;
)+

// create func
pub fn create(buf: &[u8]) -> Ret<(Box<dyn Transaction>, usize)> {
    // println!("----- transactions.parse start ------ {}", seek);
    let bts = buf_clip_mvsk!(buf, 1);
    let ty = bts[0] as u8;
    let seek = 1;
    // println!("----- transactions. typev.value()------ {} {}", seek, typev.value());
    match ty {
    $(
        $trstype => {
            let (trs, mvsk) = <$class>::create(buf) ? ;
            Ok((Box::new(trs), mvsk))
        },
    )+
    _ => Err(format!("Transaction Type <{}> not find.", ty))
    }
}

    )
}
