
#[macro_export]
macro_rules! panic_never_call_this {
    ()=>( panic!("never call this") )
}

#[macro_export]
macro_rules! must_have {
    ( $tip:expr, $value:expr ) => (
        match $value {
            None => return errf!("{} not find", $tip),
            Some(a) => a,
        }
    )
}
