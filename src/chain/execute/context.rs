


pub struct ExecEnvObj<'a> {
    fastsync: bool,
    pdhei: u64,
    pdhash: Hash,
    mainaddr: Address,
    tx: &'a dyn TransactionRead,
    // extcaller: Option<*mut ExecCaller<'a>>,
    // outstorer: Option<*mut ExecCaller<'a>>,
    // vm
    vmobj: Option<&'a mut dyn VMIvk>,
    check_sign_cache: HashMap<Address, Ret<bool>>,
}


impl ExecEnvObj<'_> {

    pub fn new<'a>(
        pdhei: u64, 
        tx: &'a dyn TransactionRead,
    ) -> ExecEnvObj<'a> {

        ExecEnvObj {
            fastsync: false,
            pdhei: pdhei,
            pdhash: Hash::default(),
            mainaddr: tx.address().unwrap(),
            tx,
            // extcaller: None,
            // outstorer: None,
            vmobj: None,
            check_sign_cache: HashMap::new(),
        }
    }


}


impl ExecContext for ExecEnvObj<'_> {

    fn pending_height(&self) -> u64 {
        self.pdhei
    }
    fn pending_hash(&self) -> &Hash {
        &self.pdhash
    }
    fn tx_fee(&self) -> &Amount {
        self.tx.fee()
    }
    fn main_address(&self) -> &Address {
        &self.mainaddr
    }
    fn addr_list(&self) -> &AddrOrList {
        &self.tx.addrlist()
    }
    fn call_depth(&self) -> u32 {
        0
    }
    fn fast_sync(&self) -> bool {
        self.fastsync
    }
    fn actions(&self) -> &Vec<Box<dyn Action>> { 
        self.tx.actions()
    }
    
    fn check_signature(&mut self, adr: &Address) -> Ret<bool> {
        if adr.version() != ADDRVER_PRIVAKEY {
            return errf!("Address {} is not PRIVAKEY type", adr.readable())
        }
        if self.check_sign_cache.contains_key(adr) {
            return self.check_sign_cache[adr].clone()
        }
        let isok = transaction::verify_target_signature(adr, self.tx);
        self.check_sign_cache.insert(*adr, isok.clone());
        isok
    }

    fn vm(&mut self) -> Ret<&mut dyn VMIvk> {
        match self.vmobj {
            Some(..) => Ok(*self.vmobj.as_mut().unwrap()),
            None => errf!("tx type or gas limit error: cannot create VM machine")
        }
    }

    fn syscall_check_true(&mut self, adr: &Address, f: u8, iptv: Vec<u8>) -> RetErr {
        if adr.version() != ADDRVER_CONTRACT {
            return Ok(()) // not contract address, ingore
        }
        let rtv = self.vm()?.sytm_call(adr, f, iptv)?;
        if rtv.len()==1 && rtv[0] == 1 {
            return Ok(()) // return true
        }
        // false
        errf!("contract {} system call <{:?}> return false", adr.readable(), f)
    }
}


/****************************************************/


pub struct ExecCaller<'a> {
    ctx: *mut ExecEnvObj<'a>,
    bst: &'a mut dyn State, 
    sto: &'a dyn Store, 
}

impl ExecCaller<'_> {

    pub fn new<'a>(
        ctx: *mut ExecEnvObj<'a>,
        bst: &'a mut dyn State, 
        sto: &'a dyn Store, 
    ) -> ExecCaller<'a> {

        ExecCaller {
            ctx,
            bst, 
            sto, 
        }
    }

    fn exec(&mut self, act: &dyn Action, depth: i8) -> Ret<(i64, Vec<u8>)> {
        unsafe { act.execute(&mut *self.ctx, self.bst, self.sto, depth) }
    }

}


impl ExtActCaller for ExecCaller<'_> {

    fn call(&mut self, kind_and_body: Vec<u8>, depth: i8) -> Ret<(i64, Vec<u8>)> {
        let (act, sk) = action::create(&kind_and_body)?;
        if sk != kind_and_body.len() {
            return Err("action data length error".to_owned())
        }
        self.exec(act.as_ref(), depth)
    }
}


impl OutStoragerRead for ExecCaller<'_> {
    fn get(&self, key: &[u8]) -> Ret<Option<Vec<u8>>> {
        Ok( self.bst.get_at(key).map(|d|d.as_ref().to_vec()) )
    }
}


impl OutStorager for ExecCaller<'_> {
    fn del(&mut self, key: &[u8]) -> RetErr {
        self.bst.del_at(key);
        Ok(())
    }
    fn set(&mut self, key: &[u8], value: Vec<u8>) -> RetErr {
        self.bst.set_at(key, value);
        Ok(())
    }
}


impl OutContext for ExecCaller<'_> { }


