
pub trait Action {
    fn get_kind(&self) -> u16;
    fn is_burning_90_persent_fee(&self) -> bool { false }
    fn request_need_sign_addresses(&self) -> HashSet<Address> { HashSet::new() }
}

