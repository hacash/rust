
impl Account {

    // create
    
    pub fn create_randomly() -> Result<Account, Error> {
        loop {
            let mut data = [0u8; 32];
            if let Err(e) = getrandom::getrandom(&mut data) {
                return Err(e.to_string())
            }
            // println!("{:?}", data)
            if data[0] < 255 {
                return Account::create_by_secret_key_value(data)
            }
        }
    }

    pub fn create_by(pass: &str) -> Result<Account, Error> {
        // is private key
        if pass.len() == 64{
            if let Ok(bts) = hex::decode(pass.clone()) {
                return Account::create_by_secret_key_value(bts.try_into().unwrap())
            }
        }
        // is passward
        return Account::create_by_password(pass)
    }

    pub fn create_by_password(pass: &str) -> Result<Account, Error> {
        let dt = sha2(pass);
        Account::create_by_secret_key_value(dt)
    }

    pub fn create_by_secret_key_value(key32: [u8; 32]) -> Result<Account, Error> {
        let kkk = key32.to_vec();
        // if(kkk.len()!=32) {
        //     return errf!("create_account_by_secret_key param key32 length must be 32 but got {}.", kkk.len()));
        // }
        if kkk[0] == 255 && kkk[1] == 255 && kkk[2] == 255 && kkk[3] == 255 {
            return Err("not support secret_key, change one and try again.".to_string());
        }
        let pk: [u8; util::SECRET_KEY_SIZE] = kkk.try_into().unwrap();
        match SecretKey::parse(&pk) {
            Err(e) => Err(e.to_string()),
            Ok(sk) => Ok(Account::create_by_secret_key(&sk)),
        }
    }

    fn create_by_secret_key(seckey: &SecretKey) -> Account {
        let pubkey = PublicKey::from_secret_key(seckey);
        let address = Account::get_address_by_public_key( pubkey.serialize_compressed() );
        let addrshow = Account::to_readable(&address);
        Account {
            secret_key: seckey.clone(),
            public_key: pubkey,
            address: address,
            address_readable: addrshow,
        }
    }


    pub fn get_address_by_public_key(pubkey: [u8; 33]) -> [u8; ADDRESS_SIZE] {
        // serialize_compressed
        let dt = sha2(pubkey);
        let dt = ripemd160(dt);
        let version = 0;
        let mut addr: [u8; 21] = [version; ADDRESS_SIZE];
        addr[1..].copy_from_slice(&dt[..]);
        addr
    }


    pub fn to_readable(addr: &[u8; ADDRESS_SIZE]) -> String {
        let version = addr[0];
        addr[1..].to_base58check(version)
    }


}
