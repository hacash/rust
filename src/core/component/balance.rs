


// AddrHac
create_combine_field_struct_and_impl!("AddrHac", AddrHac,
	address: Address,
	amount : Amount,
);

// HacAndSat
create_combine_field_struct_and_impl!("HacSat", HacSat, 
	amount : Amount,
	satoshi: OptionalSatoshi,
);

// AddrHacSat
create_combine_field_struct_and_impl!("AddrHacSat", AddrHacSat, 
	address: Address,
	hacsat : HacSat,
);


