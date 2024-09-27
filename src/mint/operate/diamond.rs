
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
        format!("diamond {}", hacd_name.readable()),
        state.diamond(hacd_name));
    if diaitem.status != DIAMOND_STATUS_NORMAL {
        return errf!("diamond {} has been mortgaged and cannot be transferred", hacd_name.readable())
    }
    if *addr_from != diaitem.address {
        return errf!("diamond {} not belong to address {}", hacd_name.readable(), addr_from.readable())
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
    let mut cost = Amount::default(); // zero
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

/* 
* return total cost
*/
pub fn engraved_clean_one_diamond(pending_height: u64, state: &mut MintState, store: &MintStoreDisk, addr :&Address, diamond: &DiamondName) -> Ret<Amount> {

    let mut diasto = check_diamond_status(state, addr, diamond)?;
    let diaslt = must_have!(format!("diamond {}", diamond.readable()), store.diamond_smelt(&diamond));
    // check
    if *diasto.inscripts.count() <= 0 {
        return errf!("cannot find any inscriptions in HACD {}", diamond.readable())    }

    // burning cost bid fee
    let cost = Amount::from_mei(diaslt.average_bid_burn.uint() as i64)?;
	// do clean
    diasto.prev_engraved_height = BlockHeight::from(0);
    diasto.inscripts = Inscripts::default();
	// save
	state.set_diamond(diamond, &diasto);

	// ok finish
	Ok(cost)
}


/**
* diamond owned push or drop
*/
pub fn diamond_owned_push_one(state: &mut MintState, address: &Address, name: &DiamondName) {
    let mut owned = state.diamond_owned(address).unwrap_or_default();
    owned.push_one(name);
    state.set_diamond_owned(address, &owned);
}

pub fn diamond_owned_move(state: &mut MintState, from: &Address, to: &Address, list: &DiamondNameListMax200) -> RetErr {
    // do drop
    let mut from_owned = state.diamond_owned(from);
    if let None = from_owned {
        return errf!("from diamond owned form not find")
    }
    let mut from_owned = from_owned.unwrap();
    let blsnum = from_owned.drop(list)?;
    if blsnum > 0 {
        state.set_diamond_owned(from, &from_owned);
    }else{
        state.del_diamond_owned(from);
    }
    // do push
    let mut to_owned = state.diamond_owned(to).unwrap_or_default();
    to_owned.push(list);
    state.set_diamond_owned(to, &to_owned);
    Ok(())
}

