
// ************ Diamond ***********

pub const DIAMOND_NAME_VALID_CHARS: &[u8; 16]  = b"WTYUIAHXVMEKBSZN";
pub type DiamondName = Fixed6;
pub type DiamondNumber = Uint3;
pub type DiamondVisualGene = Fixed10;

impl DiamondName {

    pub fn name(&self) -> String {
        String::from_utf8(self.serialize()).unwrap()
    }

    pub fn is_valid(stuff: &[u8]) -> bool {
        let v = stuff;
        if v.len() != 6 {
            return false
        }
        // check in array
        for i in v {
            let mut ok = false;
            for a in DIAMOND_NAME_VALID_CHARS {
                if i == a {
                    ok = true;
                    break
                }
            }
            if !ok {
                return false
            }
        }
        true
    }
}

// ******** DiamondNumberOptional ********

StructFieldOptional!{ DiamondNumberOptional, 
    diamond_number, DiamondNumber
}
