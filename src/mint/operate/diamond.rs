
pub fn hacd_move_one_diamond(state: &mut MintState, addr_from: &Address, addr_to: &Address, hacd_name: &DiamondName) -> RetErr {
    if addr_from == addr_to {
		return errf!("cannot transfer to self")
    }
    // query
    let mut diaitem = check_diamond_status(state, addr_from, hacd_name)?;
	// transfer diamond
    diaitem.address = addr_to.clone();
    state.set_diamond(hacd_name, &diaitem);
    // ok
    Ok(())
}


pub fn check_diamond_status(state: &mut MintState, addr_from: &Address, hacd_name: &DiamondName) -> Ret<DiamondSto> {
    // query
    let diaitem = must_have!(
        format!("diamond {}", hacd_name.to_string()),
        state.diamond(hacd_name));
    if diaitem.status != DIAMOND_STATUS_NORMAL {
        return errf!("diamond {} has been mortgaged and cannot be transferred", hacd_name.to_string())
    }
    if *addr_from != diaitem.address {
        return errf!("diamond {} not belong to address {}", hacd_name.to_string(), addr_from.readable())
    }
    // ok
    Ok(diaitem)
}



/**
* 
* return total cost
*/
pub fn engraved_one_diamond(pending_height: u64, state: &mut MintState, store: &MintStoreDisk, addr :&Address, diamond: &DiamondName, content: &BytesW1) -> Ret<Amount> {

    let mut diasto = check_diamond_status(state, addr, diamond)?;
    
    // check height
    let prev_insc_hei = diasto.prev_engraved_height.uint();
    let check_prev_block = 1000u64;
    if prev_insc_hei + check_prev_block > pending_height {
        return errf!("only one inscription can be made every {} blocks", check_prev_block)
    }

    // check insc
    let haveng = diasto.inscripts.count().uint();
    if haveng >= 200 {
        return errf!("maximum inscriptions for one diamond is 200")
    }

    let diaslt = must_have!(format!("diamond {}", diamond.readable()), store.diamond_smelt(&diamond));

    // cost
    let mut cost = Amount::new(); // zero
	if haveng >= 10 {
		// burning cost bid fee 1/10 from 11 insc
		cost = Amount::from_i64(diaslt.average_bid_burn.uint() as i64, 247)?;
	}

	// do engraved
    diasto.prev_engraved_height = BlockHeight::from(pending_height);
    diasto.inscripts.push(content.clone());
	// save
	state.set_diamond(diamond, &diasto);

	// ok finish
	Ok(cost)
}

/**
* diamond owned push or drop
*/
pub fn diamond_owned_push_one(state: &mut MintState, address: &Address, name: &DiamondName) {
    let mut owned = state.diamond_owned(address);
    if let None = owned {
        owned = Some(DiamondOwnedForm::new());
    }
    let mut owned = owned.unwrap();
    owned.push_one(name);
    state.set_diamond_owned(address, &owned);
}

pub fn diamond_owned_move(state: &mut MintState, from: &Address, to: &Address, list: &DiamondNameListMax200) -> RetErr {
    // do drop
    let mut from_owned = state.diamond_owned(from);
    if let None = from_owned {
        return errf!("from diamonds not find")
    }
    let mut from_owned = from_owned.unwrap();
    from_owned.drop(list)?;
    state.set_diamond_owned(from, &from_owned);
    // do push
    let mut to_owned = state.diamond_owned(to);
    if let None = to_owned {
        to_owned = Some(DiamondOwnedForm::new());
    }
    let mut to_owned = to_owned.unwrap();
    to_owned.push(list);
    state.set_diamond_owned(to, &to_owned);
    Ok(())
}

