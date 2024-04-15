
macro_rules! pubFnTransactionsTypeDefineCreate {
    ( $( $trstype:ident, $typev:expr, $class:ty )+ ) => (

// kind define
$(
    pub const $trstype: u8 = $typev;
)+

// create func
pub fn create(buf: &[u8]) -> Ret<(Box<dyn Transaction>, usize)> {
    // println!("----- transactions.parse start ------ {}", hex::encode(buf));
    let bts = buf_clip_mvsk!(buf, 1);
    let ty = bts[0] as u8;
    let seek = 0;
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

impl $class {

    fn hash_ex(&self, adfe: Vec<u8>) -> Hash {
        let stuff = vec![
            self.ty.serialize(),
            self.timestamp.serialize(),
            self.address.serialize(),
            adfe, /* self.fee.serialize()*/
            self.actions.serialize()
        ].concat();
        let hx = x16rs::calculate_hash(stuff);
        Hash::must(&hx[..])
    }

}

impl TransactionRead for $class {
    
    fn hash(&self) -> Hash {
        self.hash_ex(vec![]) // no fee field
    }
    
    fn hash_with_fee(&self) -> Hash {
        self.hash_ex(self.fee.serialize()) // with fee
    }

    fn ty(&self) -> u8 {
        self.ty.to_u8()
    }

    fn address(&self) -> &Address {
        &self.address
    }
    fn fee(&self) -> &Amount {
        &self.fee
    }

    fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }

    fn action_count(&self) -> u16 {
        self.actions.count().to_u64() as u16
    }
    fn actions(&self) -> &Vec<Box<dyn VMAction>> {
        self.actions.list()
    }

    fn signs(&self) -> &Vec<Sign> {
        self.signs.list()
    }
    
    // burn_90_percent_fee
    fn burn_90(&self) -> bool {
        for act in self.actions() {
            if act.as_ext().burn_90() {
                return true // burn
            }
        }
        false // not
    }

    // fee_miner_received
    fn fee_got(&self) -> Amount {
        let mut gfee = self.fee().clone();
        if self.burn_90() && gfee.unit() > 1 {
            gfee.unit_sub(1); // butn 90
        }
        gfee
    } 
    
}

impl Transaction for $class {
    fn as_read(&self) -> &dyn TransactionRead {
        self
    }
}

impl TxExec for  $class {

    fn execute(&self, blkhei: u64, sta: &mut dyn State) -> RetErr {
        // check BlockHeight more than 20w trs.Fee.Size() must less than 6 bytes.
        if blkhei > 20_0000 && self.fee.size() > 2+4 {
            return errf!("tx fee size cannot be more than 6 bytes when block height abover 200,000")
        }
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
        if exhei > 0 { // have tx !!!
            // handle hacash block chain bug start
            let bugtx = Hash::from_hex(b"f22deb27dd2893397c2bc203ddc9bc9034e455fe630d8ee310e8b5ecc6dc5628");
            if exhei == 63448 && txhx == bugtx {
                // pass the BUG
            }else{
                return errf!("tx {} already exist in height {}", txhx, exhei)
            }
            // handle bug end
        }
        // save exist mark
        exiobj.height = BlockHeight::from(blkhei);
        state.set_txexist(&txhx, &exiobj);
        // sub fee
        let feeadr = self.address();
        let amt = self.fee();    
        // println!("tx execute pay fee from {} amount {}", feeadr.to_readable(), amt.to_fin_string());
        operate::hac_sub(&mut state, feeadr, amt) ? ;
        Ok(())
    }

}



    )
}

