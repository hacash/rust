

pub enum Bytes {
    Raw(RawBytes),
    Mem(Vec<u8>),
}


impl ::std::ops::Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            Bytes::Raw(d) => d.deref(),
            Bytes::Mem(d) => d.deref(),
        }
    }
}


impl ::std::borrow::Borrow<[u8]> for Bytes {
    fn borrow(&self) -> &[u8] {
        match self {
            Bytes::Raw(d) => d.borrow(),
            Bytes::Mem(d) => d.borrow(),
        }
    }
}


impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        match self {
            Bytes::Raw(d) => d.borrow(),
            Bytes::Mem(d) => d.borrow(),
        }
    }
}

impl From<Bytes> for Vec<u8> {
    fn from(bytes: Bytes) -> Self {
        match bytes {
            Bytes::Raw(d) => d.as_ref().to_owned(),
            Bytes::Mem(d) => d.clone(),
        }
    }
}

impl From<Bytes> for Box<[u8]> {
    fn from(bytes: Bytes) -> Self {
        match bytes {
            Bytes::Raw(d) => d.as_ref().to_owned().into_boxed_slice(),
            Bytes::Mem(d) => d.clone().into_boxed_slice(),
        }
    }
}



/* 
pub enum DB {
    Disk(LevelDB),
    Memory(MemoryDB),
}

impl DB {
    // get if find, bool is not check base
    pub fn get(&self, k: &[u8]) -> (Option<Vec<u8>>, bool) {
        if let DB::Memory(mem) = self {
            let v = mem.get(k);
            if let None = v {
                return (None, false) // not find, check base
            }
            let v = v.unwrap();
            if let MemdbItem::Delete = v {
                return (None, true) // del mark, not check base
            }
            if let MemdbItem::Value(v) = v {
                return (Some(v.to_vec()), true) // find, not check base 
            }
        }else if let DB::Disk(ldb) = self {
            return (ldb.get(k), true) // leveldb own is base, not check base
        }
        // not find, check base
        (None, false)
    }

    // set
    pub fn set(&mut self, k: &[u8], v: &[u8]) {
        // must do it in mem db
        if let DB::Memory(db) = self {
            db.set(k, v);
        } else {
            panic_never_call_this!()
        }
    }

    // del
    pub fn del(&mut self, k: &[u8]) {
        // must do it in mem db
        if let DB::Memory(db) = self {
            db.del(k);
        } else {
            panic_never_call_this!()
        }
    }
}


*/