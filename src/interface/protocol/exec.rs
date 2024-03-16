
pub trait ActionExecEnv {
    fn state<'a>(&'a self) -> Box<&'a dyn State>; 
}