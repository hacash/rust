

pub trait StateDB {
    fn get(&self, p: &str, k: &impl Serialize) -> Option<Vec<u8>>;
    fn set(&self, p: &str, k: &impl Serialize, v: &impl Serialize);
    fn del(&self, p: &str, k: &impl Serialize);

}

pub trait StateRead {

    // if not find return false
    fn load(&self, p: &str, k: &impl Serialize, v: &mut impl Parse) -> bool;

}

pub trait State {

    fn init(&self);

    fn save(&self, p: &str, k: &impl Serialize, v: &impl Serialize);
    fn flush(&self); // write data to disk and remove mem db

    fn fork_next(&self) -> impl State;
}



