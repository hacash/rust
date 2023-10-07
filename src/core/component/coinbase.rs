
// CoinbaseExtendDataV1
create_combine_field_struct_and_impl!{ "CoinbaseExtendDataV1", CoinbaseExtendDataV1, 
	miner_nonce:   Hash
	witness_count: Uint1 // Number of voting witnesses
}

// CoinbaseExtend
create_optional_field_struct_and_impl!{ "CoinbaseExtend", CoinbaseExtend, datas_v1, CoinbaseExtendDataV1 }

