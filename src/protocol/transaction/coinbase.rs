
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

    fn hash(&self) -> Hash { 
        let stuff = self.serialize();
        let hx = x16rs::calculate_hash(stuff);
        Hash::must(&hx[..])
    }
    
    fn hash_with_fee(&self) -> Hash {
        self.hash()
    }

    
    
}

impl Transaction for TransactionCoinbase {

}