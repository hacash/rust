
// CoinbaseExtendDataV1
StructFieldStruct!{ CoinbaseExtendDataV1, 
	miner_nonce   : Fixed32
	witness_count : Uint1 // Number of voting witnesses
}

// CoinbaseExtend
StructFieldOptional!{ CoinbaseExtend,
    datas_v1, CoinbaseExtendDataV1
}


// coinbase
StructFieldStruct!{ TransactionCoinbase,
    ty      : Uint1
    address : Address
    reward  : Amount
    message : StringTrim16
    extend  : CoinbaseExtend
}


impl TransactionRead for TransactionCoinbase {
    
}

impl Transaction for TransactionCoinbase {

}