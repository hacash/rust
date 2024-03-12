


pub trait VM {
    fn init(
        
        _: &dyn CodeLoader,
    ) -> Option<Error> { panic_never_call_this!(); }

}