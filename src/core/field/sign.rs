
// Sign Item
StructFieldStruct!{ Sign, 
	publickey: Fixed33
	signature: Fixed64
}


// SignCheckData
StructFieldStruct!{ SignCheckData, 
	signdata: Sign
	stuffstr: BytesW2
}


// SignList MaxLen 255
StructFieldList!(SignListW1, count, Uint1, signs, Sign);


// SignList MaxLen 65535
StructFieldList!(SignListW2, count, Uint2, signs, Sign);



