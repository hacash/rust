
//  pub struct + Field + Serialize + Describe
create_combine_class_and_impl_entire_Field_trait! { TransactionCoinbase, 
	ty: Uint1,
	address: Address,
	reward: Amount,
    message: StringTrim16,
    extend: CoinbaseExtend
}

impl TransactionRead for TransactionCoinbase {

    fn get_type(&self) -> u8 {
        self.ty.to_u8()
    }

    fn get_address(&self) -> &Address {
        &self.address
    }

    fn get_reward(&self) -> &Amount {
        &self.reward
    }

    fn get_message(&self) -> &StringTrim16 {
        &self.message
    }
}



impl Transaction for TransactionCoinbase {

    fn verify_all_signs(&self) -> Option<Error> { 
        None
    }
}


impl TransactionCoinbase {

    
}


