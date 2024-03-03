
pub trait Action : VMAction {
    fn get_kind(&self) -> u16;
}

