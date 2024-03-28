

#[macro_export] 
macro_rules! StructFieldList {
    ($class: ident, $count: ident, $count_type: ty, $value: ident, $value_type: ty) => (


#[derive(Clone)]
pub struct $class  {
	$count: $count_type,
	$value: Vec<$value_type>,
}

impl Index<usize> for $class {
    type Output = $value_type;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.$value[idx]
    }
}

impl IndexMut<usize> for $class {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output{
        &mut self.$value[idx]
    }
}

impl Parse for $class {

    fn parse(&mut self, buf: &[u8], seek: usize) -> Result<usize, Error> {
        let mut seek = self.$count.parse(buf, seek) ?;
        let count = self.$count.to_u64() as usize;
        self.$value = Vec::new();
        for _ in 0..count {
            let obj: $value_type;
            (obj, seek) = <$value_type>::create(&buf[seek..]) ?;
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

   fn new() -> $class {
        $class {
            $count: <$count_type>::from_uint(0),
            $value: Vec::new(),
        }
   }

    // must & create function
    fnFieldMustCreate!($class);

}


impl $class {

	pub fn len(&self) -> usize {
		self.count() as usize
	}

	pub fn count(&self) -> u64 {
		self.$count.to_u64()
	}

	pub fn value(&self) -> &Vec<$value_type> {
		&self.$value
	}

	pub fn push(&mut self, v: $value_type) -> Option<Error> {
        if self.$count.to_usize() + 1 > <$count_type>::max() as usize {
            return Some(s!("append size overflow"))
        }
		self.$count += 1u8;
        self.$value.push(v);
        None
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

}






	)
}



// test
StructFieldList!(TestFieldList9375649365, count, Uint1, lists, Uint1);

