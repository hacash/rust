
pub trait Action : Serialize + Describe {
    // fn new() -> Self where Self: Sized;
    fn create(_: &Vec<u8>, _: usize) -> Result<(Self, usize), Error> where Self: Sized { panic!("") }
}


