
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

    fn message(&self) -> &StringTrim16 {
        &self.message
    }

    // call ret error
    
    fn verify_signature(&self) -> RetErr {
        errf!("cannot call verify_signature() in coinbase tx")
    }
    
}

impl Transaction for TransactionCoinbase {
    fn as_read(&self) -> &dyn TransactionRead {
        self
    }

    fn set_nonce(&mut self, nonce: Hash) { 
        match &mut self.extend.datas_v1 {
            Some(ref mut d) => d.miner_nonce = nonce,
            _ => (), // do nothing
        };
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
