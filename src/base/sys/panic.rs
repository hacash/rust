
#[macro_export]
macro_rules! panic_never_call_this {
    ()=>{ panic!("never call this") };
}
