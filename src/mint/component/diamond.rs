
/**
 * Diamond Status
 */
pub const DIAMOND_STATUS_NORMAL: Uint1               = Uint1::cons([1]);
pub const DIAMOND_STATUS_LENDING_TO_SYSTEM: Uint1    = Uint1::cons([2]);
pub const DIAMOND_STATUS_LENDING_TO_USER: Uint1      = Uint1::cons([3]);


/**
 * Diamond Inscripts
 */
 StructFieldList!(Inscripts, 
	count, Uint1, lists, BytesW1
);



/**
 * Diamond
 */
 StructFieldStruct!(DiamondItem, 
	status    : Uint1
	address   : Address
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
	// custom_message            : HashOptional
	average_bid_burn          : Uint2
	visual_gene               : Fixed10
);





