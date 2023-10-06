
// Sign Item
create_combine_field_struct_and_impl!("Sign",Sign, 
	publickey: Fixed33
	signature: Fixed64
);


// SignCheckData
create_combine_field_struct_and_impl!("SignCheckData", SignCheckData, 
	signdata: Sign
	stuffstr: BytesMaxLen2
);


// SignListMax255
create_list_field_struct_and_impl!("SignListMax255", SignListMax255, count, Uint1, signs, Sign);


// SignListMax65535
create_list_field_struct_and_impl!("SignListMax65535", SignListMax65535, count, Uint2, signs, Sign);



