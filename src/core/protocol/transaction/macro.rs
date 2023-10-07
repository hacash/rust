
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

}


impl Transaction for $class {

    fn verify_all_signs(&self) -> Option<Error> { 
        None
    }

}



    )
}

