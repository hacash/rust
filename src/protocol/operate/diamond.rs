

macro_rules! fnDiamondOperateCommon{
    ($func_name: ident, $addr:ident, $hacd:ident, $oldhacd:ident, $newhacdblock:block) => (

pub fn $func_name(state: &mut CoreState, $addr: &Address, $hacd: &DiamondNumber) -> Ret<DiamondNumber> {
    let mut userbls = state.balance( $addr ).unwrap_or_default();
    let $oldhacd = &userbls.diamond.to_diamond();
    /* -------- */
    let newhacd = $newhacdblock;// operate
    /* -------- */
    // save
    userbls.diamond = DiamondNumberAuto::from_diamond( &newhacd );
    state.set_balance($addr, &userbls);
    Ok(newhacd)
}

    )
}


/**************************** */

fnDiamondOperateCommon!(hacd_add, addr, hacd, oldhacd, {
    // do add
    *oldhacd + *hacd
});

fnDiamondOperateCommon!(hacd_sub, addr, hacd, oldhacd, {  
    // check
    if oldhacd.uint() < hacd.uint().into() {
		return errf!("do hacd_sub error: address {} diamond {} not enough, need {}", 
            addr.readable(), oldhacd.uint(), hacd.uint())
    }
    // do sub
    *oldhacd - *hacd
});



/**************************** */


pub fn hacd_transfer(ctx: &mut dyn ExecContext, state: &mut CoreState, 
    addr_from: &Address, addr_to: &Address, hacd: &DiamondNumber, dlist: &DiamondNameListMax200
) -> Ret<Vec<u8>> {
    if addr_from == addr_to {
		return errf!("cannot transfer to self")
    }
    // check contract
    /*use vm::rt::SystemCallType::*;
    let amtv = dlist.form(); // name bytes vec
    ctx.syscall_check_true(addr_from, PermitHACD  as u8, amtv.clone())?;
    ctx.syscall_check_true(addr_to,   PayableHACD as u8, amtv)?;*/
    // do transfer
    hacd_sub(state, addr_from, hacd)?;
    hacd_add(state, addr_to, hacd)?;
    // ok
    Ok(vec![])
}

