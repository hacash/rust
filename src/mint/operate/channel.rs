
// close default
pub fn close_channel_default(pdhei: u64, sta: &mut dyn State, channel_id: &ChannelId, paychan: &ChannelSto
) -> Ret<Vec<u8>> {
    close_channel_with_distribution(
        pdhei, sta, channel_id, paychan, 
        &paychan.left_bill.hacsat.amount,
        &paychan.right_bill.hacsat.amount,
        &paychan.left_bill.hacsat.satoshi.value(),
        &paychan.right_bill.hacsat.satoshi.value(),
        false,
    )
}


/**
 * close
 * pdhei = pending height
 */
pub fn close_channel_with_distribution(pdhei: u64, sta: &mut dyn State, channel_id: &ChannelId, 
    paychan: &ChannelSto, 
    left_amt: &Amount,  right_amt: &Amount,
    left_sat: &Satoshi, right_sat: &Satoshi,
    is_final_closed: bool,
) -> Ret<Vec<u8>> {

    // check
    if paychan.status != CHANNEL_STATUS_OPENING {
        return errf!("channel status is not opening")
    }
    let left_addr = &paychan.left_bill.address;
    let right_addr = &paychan.right_bill.address;
	if left_amt.is_negative() || right_amt.is_negative() {
		return errf!("channel distribution amount cannot be negative.")
	}
    let ttamt = paychan.left_bill.hacsat.amount.add(&paychan.right_bill.hacsat.amount)?;
    if  left_amt.add(right_amt)?.not_equal(&ttamt) {
        return errf!("HAC distribution amount must equal with lock in.")
    }
    let ttsat = paychan.left_bill.hacsat.satoshi.value() + paychan.right_bill.hacsat.satoshi.value();
    if *left_sat + *right_sat != ttsat {
        return errf!("BTC distribution amount must equal with lock in.")
    }
    // total supply
    let mut ttcount = {
        MintState::wrap(sta).total_count()
    };
    ttcount.opening_channel -= 1u64;
    // core state
    let mut core_state = CoreState::wrap(sta);
    // do close
    if ttamt.is_positive() {
        // calculate_interest
        let (newamt1, newamt2) = calculate_interest_of_height(
            pdhei, paychan.belong_height.to_u64(), 
            paychan.interest_attribution, left_amt, right_amt
        )?;
        let ttnewhac = newamt1.add(&newamt2) ?;
        if ttnewhac.less_than( &ttamt ) {
            return errf!("interest calculate error!")
        }
        let ttiesthac =  ttnewhac.sub(&ttamt) ? .to_zhu_unsafe() as u64;
        ttcount.channel_interest_zhu += ttiesthac;
        ttcount.channel_deposit_zhu -= ttamt.to_zhu_unsafe() as u64;
        if newamt1.is_positive() {
            hac_add(&mut core_state, left_addr, &newamt1)?;
        }
        if newamt2.is_positive() {
            hac_add(&mut core_state, right_addr, &newamt2)?;
        }
    }
    let ttsatn = ttsat.to_u64();
    if ttsatn > 0 {
        ttcount.channel_deposit_sat -= ttsatn;
        if left_sat.to_u64() > 0 {
            sat_add(&mut core_state, left_addr, left_sat)?;
        }
        if right_sat.to_u64() > 0 {
            sat_add(&mut core_state, right_addr, right_sat)?;
        }
    }
    drop(core_state);

    let mut state = MintState::wrap(sta);
    // save channel
    let distribution = ClosedDistributionDataOptional::must(ClosedDistributionData{
        left_bill: HacSat{
            amount: left_amt.clone(),
            satoshi: SatoshiOptional::must(left_sat.clone()),
        }
    });
    let mut savechan = paychan.clone();
    savechan.status = match is_final_closed {
        true => CHANNEL_STATUS_FINAL_ARBITRATION_CLOSED,
        false => CHANNEL_STATUS_AGREEMENT_CLOSED,
    };
    savechan.if_distribution = distribution;
    // save channel and count
    state.set_channel(&channel_id, &savechan);
    state.set_total_count(&ttcount);
    // ok finish
    Ok(vec![])
}


