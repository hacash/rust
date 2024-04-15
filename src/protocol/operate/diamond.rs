

macro_rules! fnDiamondOperateCommon{
    ($func_name: ident, $addr:ident, $hacd:ident, $oldhacd:ident, $newhacdblock:block) => (

pub fn $func_name(state: &mut CoreState, $addr: &Address, $hacd: &DiamondNumber) -> Ret<DiamondNumber> {
    let mut userbls;
    if let Some(b) = state.balance( $addr ) {
        userbls = b;
    } else {
        userbls = Balance::new(); // empty
    }
    let $oldhacd = &userbls.diamond.value();
    /* -------- */
    let newhacd = $newhacdblock;// operate
    /* -------- */
    // save
    userbls.diamond = DiamondNumberOptional::must(newhacd.clone());
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
    if oldhacd.uint() < hacd.uint() {
		return errf!("do hacd_sub error: address {} diamond {} not enough, need {}", 
            addr.to_readable(), oldhacd.uint(), hacd.uint())
    }
    // do sub
    *oldhacd - *hacd
});



/**************************** */


pub fn hacd_transfer(state: &mut CoreState, addr_from: &Address, addr_to: &Address, hacd: &DiamondNumber) -> RetErr {
    if addr_from == addr_to {
		return errf!("cannot transfer to self")
    }
    hacd_sub(state, addr_from, hacd) ? ;
    hacd_add(state, addr_to, hacd) ? ;
    // ok
    Ok(())
}

