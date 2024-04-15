
pub fn hacd_move_one_diamond(state: &mut MintState, addr_from: &Address, addr_to: &Address, hacd_name: &DiamondName) -> RetErr {
    if addr_from == addr_to {
		return errf!("cannot transfer to self")
    }
    // query
    let mut diaitem = must_have!(
        format!("diamond {}", hacd_name.to_string()),
        state.diamond(hacd_name));
    if diaitem.status != DIAMOND_STATUS_NORMAL {
        return errf!("diamond {} has been mortgaged and cannot be transferred", hacd_name.to_string())
    }
    if *addr_from != diaitem.address {
        return errf!("diamond {} not belong to address {}", hacd_name.to_string(), addr_from.to_readable())
    }
	// transfer diamond
    diaitem.address = addr_to.clone();
    state.set_diamond(hacd_name, &diaitem);
    // ok
    Ok(())
}
