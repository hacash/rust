
/**
 * memory db
 */
enum MemdbItem {
    Delete, // is delete
    Value(Vec<u8>),
}

impl MemdbItem {
    fn from(v: &[u8]) -> MemdbItem {
        MemdbItem::Value(v.to_vec())
    }
}

struct MemoryDB(
    HashMap<Vec<u8>, MemdbItem>
);

impl MemoryDB {
    // get
    fn get(&self, k: &[u8]) -> Option<&[u8]> {
        if let Some(v) = self.0.get(k) {
            if let MemdbItem::Value(v) = v {
                return Some(&v)
            }
        }
        // not find or delete
        None
    }

    // set
    fn set(&mut self, k: &[u8], v: &[u8]) {
        self.0.insert(k.to_vec(), MemdbItem::from(v));
    }

    // del
    fn del(&mut self, k: &[u8]) {
        self.0.insert(k.to_vec(), MemdbItem::Delete);
    }
}


