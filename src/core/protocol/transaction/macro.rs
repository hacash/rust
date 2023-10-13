
macro_rules! define_transactions_type_and_create_func {
    ( $( $typev:tt $trstype:ident: $class:ty )+ ) => (

        // kind define
        $(
            pub const $trstype: u8 = $typev;
        )+

        // parse func
        pub fn create(buf: &Vec<u8>, seek: usize) -> Result<(Box<dyn Transaction>, usize), String> {
            let (typev, _) = create_field_or_error!("transactions.create", Uint1, buf, seek);
            let ty = typev.to_u8();
            match ty {
            $(
                $trstype => {
                    let (trs, mvsk) = <$class>::create(buf, seek) ? ;
                    Ok((Box::new(trs), mvsk))
                },
            )+
            _ => Err(format!("Transaction Type <{}> not find.", ty))
            }
        }

    )
}


////////////////////////////////



macro_rules! create_common_transaction_struct{
    ($tyid: expr, $class:ident) => (


//  pub struct + Field + Serialize + Describe
create_combine_class_and_impl_entire_Field_trait! { $class, 
	ty:             Uint1
	timestamp:      Timestamp
	address:        Address
	fee:            Amount
    actions:        DynListActionMax65535
    signs:          SignListMax65535
	multisign_mark: Uint2
}


impl TransactionRead for $class {


    fn get_type(&self) -> u8 {
        self.ty.to_u8()
    }

    create_get_func_for_combine_class!{
        address: Address
        fee:     Amount
    }


    fn hash(&self) -> Hash {
        // calculate hash no fee
        let stuff = self.serialize_for_sign_no_fee();
        let hx = x16rs::calculate_hash(stuff);
        let hx = Hash::from(hx);
        hx
    }
    fn hash_with_fee(&self) -> Hash {
        // calculate hash with fee
        let stuff = self.serialize_for_sign();
        let hx = x16rs::calculate_hash(stuff);
        let hx = Hash::from(hx);
        hx
    }
    

}


impl Transaction for $class {

    fn verify_all_signs(&self) -> Option<Error> { 
        Some("not yet".to_string())
    }

    fn append_action(&mut self, act: Box<dyn Action>) -> Option<Error> { 
        self.actions.append(act);
        None
    }

    fn fill_sign(&mut self, acc: &Account) -> Option<Error> { 
        let hx: Hash;
        if self.address == *acc.address() {
            hx = self.hash_with_fee();
        }else{
            hx = self.hash();
        }
        // sign
        let sg = acc.do_sign(&hx);
        // add
        self.signs.append(Sign{
            publickey: Fixed33::from_u8s(acc.public_key().serialize_compressed()),
            signature: Fixed64::from_u8s(sg),
        });
        // ok
        None

    }


}

impl $class {

    fn serialize_for_sign(&self) -> Vec<u8> {
        field_serialize_items_concat!(
            self.ty,
            self.timestamp,
            self.address, 
            self.fee,
            self.actions
        )
    }

    fn serialize_for_sign_no_fee(&self) -> Vec<u8> {
        field_serialize_items_concat!(
            self.ty,
            self.timestamp,
            self.address, 
            self.actions
        )
    }
}



    )
}

