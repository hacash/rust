

pub trait VM: Send + Sync {
    fn new(_: &IniObj, _: Arc<dyn Store>) -> Self where Self: Sized;
    fn exec(&self, _: &dyn ExecEnv, _: &mut dyn State, _: &Vec<Box<dyn VMAction>>) -> RetErr { panic_never_call_this!() }
}


