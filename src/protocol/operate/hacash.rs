
macro_rules! fnHacashOperateCommon{
    ($func_name: ident, $addr:ident, $amt:ident, $oldhac:ident,  $newamtblock:block) => (

pub fn $func_name(state: &mut CoreState, $addr: &Address, $amt: &Amount) -> Ret<Amount> {
    
    if ! $amt.is_positive() {
		return errf!("amount {} value is not positive", $amt.to_fin_string())
    }
    let mut userbls = state.balance( $addr ).unwrap_or_default();
    let $oldhac = userbls.hacash;
    /* -------- */
    let newamt = $newamtblock;// operate
    /* -------- */
    if newamt.size() > 12 {
		return errf!("address {} amount {} size {} over 12 can not to store", 
        $addr.readable(), newamt.size(), newamt.to_fin_string())
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
		return errf!("do hac_sub error: address {} balance {} not enough, need {}", 
            addr.readable(), oldhac.to_fin_string(), amt.to_fin_string())
    }
    // do sub
    oldhac.sub( amt ) ?
});



/****************************/


pub fn hac_transfer(ctx: &mut dyn ExecContext, stadb: &mut dyn State, 
    addr_from: &Address, addr_to: &Address, amt: &Amount
) -> Ret<Vec<u8>> {
	let mut state = CoreState::wrap(stadb);
    let is_trs_to_my_self = addr_from == addr_to;
    if is_trs_to_my_self {
        if ctx.pending_height() >= 20_0000 {
            // you can transfer it to yourself without changing the status, which is a waste of service fees
            hac_check(&mut state, addr_from, amt)?;
        }
        return Ok(vec![]);
    }
    // check contract
    /*use vm::rt::SystemCallType::*;
    let amtv = amt.serialize();
    ctx.syscall_check_true(addr_from, PermitHAC  as u8, amtv.clone())?;
    ctx.syscall_check_true(addr_to,   PayableHAC as u8, amtv)?;*/
	// after 200000 height, the amount transferred to self is not allowed to be greater than the available balance!
    // println!("hac_transfer hac_sub from {} to {} amount {}", addr_from.readable(), addr_to.readable(), amt.to_fin_string());
    hac_sub(&mut state, addr_from, amt)?;
    hac_add(&mut state, addr_to, amt)?;
    // ok
    Ok(vec![])
}


pub fn hac_check(state: &mut CoreState, addr: &Address, amt: &Amount) -> Ret<Amount> {
    if ! amt.is_positive() {
        return errf!("check amount is cannot empty")
    }
    if let Some(bls) = state.balance( addr ) {
        // println!("address {} balance {}", addr.readable(), bls.hacash.to_fin_string() );
        if bls.hacash >= *amt {
            return Ok(bls.hacash)
        }
    }
    errf!("address {} balance not enough need {}", addr.readable(), amt.to_fin_string() )
}






