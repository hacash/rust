static NEVER_CALL_THIS: &str = "never call this"; 

#[macro_export]
macro_rules! panic_never_call_this {
()=>( panic!(NEVER_CALL_THIS.to_string()) )
}


