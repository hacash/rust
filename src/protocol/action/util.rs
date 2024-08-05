

macro_rules! require_address_version_privkey {
    ($addr: expr) => {
        if $addr.version() != ADDRVER_PRIVAKEY {
            return errf!("address {} is not version PRIVAKEY", $addr.readable())
        }
    }
}