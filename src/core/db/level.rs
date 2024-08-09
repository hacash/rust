

use rusty_leveldb::{DB as LVDB, DBIterator, LdbIterator, WriteBatch, Options};


pub type Writebatch = WriteBatch;

pub struct LevelDB {
    ldb: Mutex<LVDB>
}


macro_rules! ldb {
    ($s: ident) => {
        $s.ldb.lock().unwrap()
    }
}

impl LevelDB {

    pub fn open(dir: &Path) -> LevelDB {
        let dir = dir.to_str().unwrap();
        let ldb = LVDB::open(dir, Options:: default()).unwrap();
        LevelDB{
            ldb: Mutex::new(ldb),
        }
    }

    pub fn get_at(&self, key: &[u8]) -> Option<Vec<u8>> {
        ldb!(self).get(key)
    }

    pub fn put(&self, key: &[u8], val: &[u8]) {
        ldb!(self).put(key, val).unwrap()
    }

    pub fn rm(&self, key: &[u8]) {
        ldb!(self).delete(key).unwrap()
    }

    pub fn write(&self, batch: Writebatch) {
        let is_sync = false;
        let mut db = ldb!(self);
        db.write(batch, is_sync).unwrap();
        db.flush();
    }

}



