
/**
 * Diamond Status
 */
pub const DIAMOND_STATUS_NORMAL                : Uint1 = Uint1::from(1);
pub const DIAMOND_STATUS_LENDING_TO_SYSTEM     : Uint1 = Uint1::from(2);
pub const DIAMOND_STATUS_LENDING_TO_USER       : Uint1 = Uint1::from(3);


/**
 * Diamond Inscripts
 */
 StructFieldList!(Inscripts, 
	count, Uint1, lists, BytesW1
);

impl Inscripts {
	pub fn array(&self) -> Vec<String> {
		let mut resv = Vec::with_capacity(self.lists.len());
		for li in &self.lists {
			let rdstr = bytes_try_to_readable_string(li.as_ref());
			resv.push(match rdstr {
				None => hex::encode(li.as_ref()),
				Some(s) => s,
			});
		}
		resv
	}
}


/**
 * Diamond
 */
 StructFieldStruct!(DiamondSto, 
	status    : Uint1
	address   : Address
	prev_engraved_height : BlockHeight
	inscripts : Inscripts
);


/**
 * DiamondSmelt
 */
 StructFieldStruct!(DiamondSmelt, 
	diamond                   : DiamondName
	number                    : DiamondNumber
	belong_height             : BlockHeight
	belong_hash               : Hash // block
	prev_hash                 : Hash // block
	miner_address             : Address
	bid_fee                   : Amount
	nonce                     : Fixed8
	// custom_message           : HashOptional
	average_bid_burn          : Uint2
	visual_gene               : Fixed10
);





