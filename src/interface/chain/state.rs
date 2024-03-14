

pub trait StateDB {
    fn get(&self, p: u8, k: &impl Serialize) -> Option<Vec<u8>>;
    fn set(&self, p: u8, k: &impl Serialize, v: &impl Serialize);
    fn del(&self, p: u8, k: &impl Serialize);

}

pub trait StateRead {

    // if not find return false
    fn load(&self, p: u8, k: &impl Serialize, v: &mut impl Parse) -> bool;

}

pub trait State {

    fn init(&self);

    fn save(&self, p: u8, k: &impl Serialize, v: &impl Serialize);
    fn flush(&self); // write data to disk and remove mem db

    fn fork_next(&self) -> impl State;
}



