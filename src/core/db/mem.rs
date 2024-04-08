
/**
 * memory db
 */
#[derive(Debug, Clone)]
pub enum MemdbItem {
    Delete, // is delete
    Value(Vec<u8>),
}

impl MemdbItem {
    pub fn from(v: &[u8]) -> MemdbItem {
        MemdbItem::Value(v.to_vec())
    }
}

pub struct MemoryDB(
    HashMap<Vec<u8>, MemdbItem>
);

impl MemoryDB {

    pub fn new() -> MemoryDB {
        MemoryDB(
            HashMap::new()
        )
    }

    pub fn iter(& self) -> MapIter<'_, Vec<u8>, MemdbItem> {
        self.0.iter()
    }

    // get
    pub fn get(&self, k: &[u8]) -> Option<&MemdbItem> {
        self.0.get(k) // find or not ret none
    }

    // set
    pub fn set(&mut self, k: &[u8], v: &[u8]) {
        self.0.insert(k.to_vec(), MemdbItem::from(v));
    }

    // del
    pub fn del(&mut self, k: &[u8]) {
        self.0.insert(k.to_vec(), MemdbItem::Delete);
    }
}


