

#[macro_export] 
macro_rules! StructFieldList {
    ($class: ident, $count: ident, $count_type: ty, $value: ident, $value_type: ty) => (


#[derive(Default, Clone, PartialEq, Eq)]
pub struct $class  {
	$count: $count_type,
	$value: Vec<$value_type>,
}

impl std::fmt::Debug for $class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"[list {}]", self.$count.to_u64())
    }
}

impl std::ops::Index<usize> for $class {
    type Output = $value_type;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.$value[idx]
    }
}

impl std::ops::IndexMut<usize> for $class {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output{
        &mut self.$value[idx]
    }
}

impl FieldReadable for $class {
    fn readable(&self) -> String {
        format!("[...]")
    }
}


impl Parse for $class {

    fn parse(&mut self, buf: &[u8], seek: usize) -> Ret<usize> {
        let mut seek = self.$count.parse(buf, seek) ?;
        let count = self.$count.to_u64() as usize;
        self.$value = Vec::new();
        for _ in 0..count {
            let (obj, mvsk) = <$value_type>::create(&buf[seek..]) ?;
            seek += mvsk;
            self.$value.push(obj);
        }
        Ok(seek)
    }

}


impl Serialize for $class {

    fn serialize(&self) -> Vec<u8> {
        let mut resdt = self.$count.serialize();
        let count = self.$count.to_u64() as usize;
        for i in 0..count {
            let mut vardt = self.$value[i].serialize();
            resdt.append(&mut vardt);
        }
        resdt
    }

    fn size(&self) -> usize {
        let mut size = self.$count.size();
        let count = self.$count.to_u64() as usize;
        for i in 0..count {
            size += self.$value[i].size();
        }
        size
    }

}

impl Field for $class {

    // must & create function
    fnFieldMustCreate!($class);

}


impl $class {

	pub fn count(&self) -> &$count_type {
		&self.$count
	}

	pub fn list(&self) -> &Vec<$value_type> {
		&self.$value
	}

	pub fn push(&mut self, v: $value_type) -> RetErr {
        if self.$count.to_usize() + 1 > <$count_type>::max() as usize {
            return errf!("append size overflow")
        }
		self.$count += 1u8;
        self.$value.push(v);
        Ok(())
	}

	pub fn append(&mut self, mut list: Vec<$value_type>) -> RetErr {
        if self.$count.to_usize() + list.len() > <$count_type>::max() as usize {
            return errf!("append size overflow")
        }
		self.$count += list.len() as u8;
        self.$value.append(&mut list);
        Ok(())
	}

	pub fn pop(&mut self) -> Option<$value_type> {
        let n = self.$count.to_u64();
        match n {
            0 => None,
            _ => {
                self.$count -= 1u32;
                self.$value.pop()
            }
        }
	}

	pub fn as_mut(&mut self) -> &mut Vec<$value_type> {
	    &mut self.$value
    }

}






	)
}



// test
StructFieldList!(TestFieldList9375649365, count, Uint1, lists, Uint1);

