

pub trait Receipt {
    fn gas_use(&self) -> u32 { 4200000000 }
    fn burn_90(&self) -> bool { false }
    fn state(&mut self) -> Box<dyn State> { panic_never_call_this!() }
    fn logs(&mut self) -> Vec<String> { panic_never_call_this!() }
}