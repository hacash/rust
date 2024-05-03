
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

/**
 * Diamond Name List
 */
 StructFieldList!(DiamondNameListMax200, 
	count, Uint1, lists, DiamondName
);


impl DiamondNameListMax200 {

    pub fn check(&self) -> Ret<u8> {
        // check len
        let setlen = self.count.uint() as u64;
        let reallen = self.lists.len() as u64 ;
        if setlen != reallen {
            return errf!("check fail: length need {} but got {}", setlen, reallen)
        }
        if reallen == 0 {
            return errf!("diamonds quantity cannot be zero")
        }
        if reallen > 200 {
            return errf!("diamonds quantity cannot over 200")
        }
        // check name
        for v in &self.lists {
            if ! DiamondName::is_valid(v.as_ref()) {
                return errf!("diamond name {} is not valid", v.readable())
            }
        }
        // success
        Ok(reallen as u8)
    }

}
