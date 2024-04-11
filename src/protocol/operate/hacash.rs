
macro_rules! fnHacashOperateCommon{
    ($func_name: ident, $addr:ident, $amt:ident, $oldhac:ident,  $newamtblock:block) => (

pub fn $func_name(state: &mut CoreState, $addr: &Address, $amt: &Amount) -> Ret<Amount> {
    
    
    if ! $amt.is_positive() {
		return Err(format!("amount {} value is not positive", $amt.to_fin_string()))
    }
    let mut userbls;
    if let Some(b) = state.balance( $addr ) {
        userbls = b;
    } else {
        userbls = Balance::new(); // empty
    }
    let $oldhac = userbls.hacash;
    /* -------- */
    let newamt = $newamtblock;// operate
    /* -------- */
    if newamt.size() > 11 {
		return Err("amount size over 11 can not to store".to_string())
    }
    // save
    userbls.hacash = newamt.clone();
    state.set_balance($addr, &userbls);
    Ok(newamt)
}

    )
}


/**************************** */

fnHacashOperateCommon!(hac_add, addr, amt, oldhac, {
    // do add
    oldhac.add( amt ) ? 
});


fnHacashOperateCommon!(hac_sub, addr, amt, oldhac, {  
    // check
    if oldhac < *amt {
		return Err(format!("do hac_sub error: address {} balance {} not enough, need {}", 
            addr.to_readable(), oldhac.to_fin_string(), amt.to_fin_string()))
    }
    // do sub
    oldhac.sub( amt ) ?
});



/****************************/


pub fn hac_transfer(env: &dyn ExecEnv, stadb: &mut dyn State, addr_from: &Address, addr_to: &Address, amt: &Amount) -> RetErr {
	let mut state = CoreState::wrap(stadb);
    let is_trs_to_my_self = addr_from == addr_to;
    if is_trs_to_my_self && env.pending_height() < 20_0000 {
        // you can transfer it to yourself without changing the status, which is a waste of service fees
		return Ok(()) 
    }
	// after 200000 height, the amount transferred to self is not allowed to be greater than the available balance!
    hac_sub(&mut state, addr_from, amt) ? ;
    hac_add(&mut state, addr_to, amt) ? ;
    // ok
    Ok(())
}


pub fn hac_check(state: &mut CoreState, addr: &Address, amt: &Amount) -> Ret<Amount> {
    if ! amt.is_positive() {
        return Err("check amount is cannot empty".to_string())
    }
    if let Some(bls) = state.balance( addr ) {
        if bls.hacash >= *amt {
            return Ok(bls.hacash)
        }
    }
    Err("address {} balance not enough".to_string())
}






