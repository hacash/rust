
// ************ Diamond ***********

pub const DIAMOND_NAME_VALID_CHARS: &[u8; 16]  = b"WTYUIAHXVMEKBSZN";
pub type DiamondName = Fixed6;
pub type DiamondNumber = Uint3;
pub type DiamondVisualGene = Fixed10;
pub type DiamondLifeGene   = Fixed32;

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

// ******** DiamondNumberOptional and Auto ********

pub type DiamondNumberAuto = AutoU64;
StructFieldOptional!{ DiamondNumberOptional, 
    diamond_number, DiamondNumber
}
impl DiamondNumberAuto {
	pub fn to_diamond(&self) -> DiamondNumber {
		DiamondNumber::from( self.uint() as u32 )
	}
	pub fn from_diamond(dia: &DiamondNumber) -> DiamondNumberAuto {
		DiamondNumberAuto::from( dia.uint() as u64 )
	}
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
    
    pub fn contains(&self, x: &[u8]) -> bool {
        for v in &self.lists {
            if x == v.as_ref() {
                return true
            }
        }
        false // not find
    }

    pub fn readable(&self) -> String {
        self.lists.iter().map(|a|a.readable()).collect::<Vec<_>>().join(",")
    }

    pub fn form(&self) -> Vec<u8> {
        self.lists.iter().map(|a|a.serialize()).collect::<Vec<_>>().concat()
    }

    pub fn hashset(&self) -> HashSet<DiamondName> {
        self.lists.iter().map(|a|a.clone()).collect::<HashSet<_>>()
    }

    pub fn from_string(stuff: &String) -> Ret<DiamondNameListMax200> {
        let s = stuff.replace(" ","").replace("\n","").replace("|","").replace(",","");
        if s.len() == 0 {
            return errf!("diamond list empty")
        }
        if s.len() % 6 != 0 {
            return errf!("diamond list format error")
        }
        let num = s.len() / 6;
        if num > 200  {
            return errf!("diamond list max 200 overflow")
        }
        let mut obj = DiamondNameListMax200::default();
        let bs = s.as_bytes();
        for i in 0 .. num {
            let x = i*6;
            let name = DiamondName::cons( bufcut!(bs, x, x+6) );
            obj.push(name);
        }
        obj.check()?;
        Ok(obj)
    }

}
