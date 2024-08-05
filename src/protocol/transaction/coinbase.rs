
// CoinbaseExtendDataV1
StructFieldStruct!{ CoinbaseExtendDataV1, 
	miner_nonce   : Hash
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

    fn ty(&self) -> u8 {
        self.ty.to_u8()
    }

    fn address(&self) -> Ret<Address> {
        Ok(self.address.clone())
    }

    fn reward(&self) -> &Amount {
        &self.reward
    }
    
}

impl Transaction for TransactionCoinbase {
    fn as_read(&self) -> &dyn TransactionRead {
        self
    }
}

impl TxExec for TransactionCoinbase {
    fn execute(&self, _: u64, sta: &mut dyn State) -> RetErr {
        let mut state = CoreState::wrap(sta);
        let rwdadr = self.address()?;
        let amt = self.reward();
        operate::hac_add(&mut state, &rwdadr, amt)?;
        Ok(())
    }
}
