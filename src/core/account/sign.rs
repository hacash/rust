

// signature
impl Account {

    // return Signature
    pub fn do_sign(&self, msg: &[u8; 32]) -> [u8; 64] {
        let msg = Message::parse(msg);
        let (s, r) = libsecp256k1::sign(&msg, &self.secret_key);
        s.serialize()
    }

    pub fn verify_signature(msg: &[u8; 32], publickey: &[u8; 33], signature: &[u8; 64]) -> bool {
        if let Ok(pubkey) = PublicKey::parse_compressed(publickey) {
            if let Ok(sigobj) = Signature::parse_standard(signature) {
                return libsecp256k1::verify(
                    &Message::parse(msg),
                    &sigobj,
                    &pubkey,
                )
            }
        }
        false
    }

}
        