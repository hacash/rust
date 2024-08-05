
pub trait Action : Field + Send + Sync + ActExec + dyn_clone::DynClone {
    fn kind(&self) -> u16 { 0 }
    fn level(&self) -> i8 { ACTLV_MAIN }
    fn gas(&self) -> i64 { 0 } // fixed gas use
    fn burn_90(&self) -> bool { false } // is_burning_90_persent_fee
    fn req_sign(&self) -> Vec<AddrOrPtr> { vec![] } // request_need_sign_addresses
}


dyn_clone::clone_trait_object!(Action);

