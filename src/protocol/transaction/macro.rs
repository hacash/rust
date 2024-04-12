
macro_rules! pubFnTransactionsTypeDefineCreate {
    ( $( $trstype:ident, $typev:expr, $class:ty )+ ) => (

// kind define
$(
    pub const $trstype: u8 = $typev;
)+

// create func
pub fn create(buf: &[u8]) -> Ret<(Box<dyn Transaction>, usize)> {
    // println!("----- transactions.parse start ------ {}", seek);
    let bts = buf_clip_mvsk!(buf, 1);
    let ty = bts[0] as u8;
    let seek = 1;
    // println!("----- transactions. typev.value()------ {} {}", seek, typev.value());
    match ty {
    $(
        $trstype => {
            let (trs, mvsk) = <$class>::create(buf) ? ;
            Ok((Box::new(trs), mvsk))
        },
    )+
    _ => Err(format!("Transaction Type <{}> not find.", ty))
    }
}

    )
}



/*************** transaction ****************/


macro_rules! DefineCommonTransaction {
    ($tyid: expr, $class:ident) => (

// Transaction Type 1 & 2
StructFieldStruct!{ $class, 
	ty        : Uint1
	timestamp : Timestamp
	address   : Address
	fee       : Amount
    actions   : DynListVMAction
    signs     : SignListW2
	ano_mark  : Uint2
}

impl TransactionRead for $class {
    
}

impl Transaction for $class {
    fn as_read(&self) -> &dyn TransactionRead {
        self
    }
}

impl TxExec for  $class {

    fn execute(&self, blkhei: u64, sta: &mut dyn State) -> RetErr {
        let mut state = CoreState::wrap(sta);
        // check tx exist
        let txhx = self.hash();
        let mut exiobj;
        if let Some(exi) = state.txexist(&txhx) {
            exiobj = exi;
        }else{
            exiobj = TxExist::new();
        }
        let exhei = exiobj.height.to_u64();
        if exhei > 0 {
            return errf!("tx {} already exist in height {}", txhx, exhei)
        }
        // save exist mark
        exiobj.height = BlockHeight::from_uint(blkhei);
        state.set_txexist(&txhx, &exiobj);
        // sub fee
        let feeadr = self.address();
        let amt = self.fee();
        operate::hac_sub(&mut state, feeadr, amt) ? ;
        Ok(())
    }
    
}



    )
}

