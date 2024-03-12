
/**
 * memory db
 */
enum MemItem {
    IsDel(bool), // is delete
    Value(Vec<u8>),
} 

type MemDB = HashMap<Vec<u8>, MemItem>;



