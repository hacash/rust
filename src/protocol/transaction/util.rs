
/**
* verify tx all needs signature
*/
pub fn verify_tx_signature(tx: &dyn Transaction) -> RetErr {
    let hx = tx.hash();
    let hxwf = tx.hash_with_fee();
    let signs = tx.signs();
    let addrs = tx.req_sign();
    let main_addr = tx.address();
    let txty = tx.ty();
    for adr in addrs {
        let mut ckhx = &hx;
        if adr == *main_addr && txty > TX_TYPE_1_DEPRECATED{
            ckhx = &hxwf;
        }
        verify_one_sign(ckhx, &adr, signs)?;
    }
    Ok(())
}


fn verify_one_sign(hash: &Hash, addr: &Address, signs: &Vec<Sign>) -> RetErr {
    for sig in signs {
        let curpubkey = sig.publickey.into_array();
        let curaddr = Account::get_address_by_public_key(curpubkey);
        if addr.into_array() == curaddr {
            if Account::verify_signature(&hash.into_array(), &curpubkey, &sig.signature.into_array()) {
                return Ok(())
            }
        }
    }
    errf!("{} verify signature failed", addr.to_readable())
}