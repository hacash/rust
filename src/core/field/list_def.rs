

#[macro_export] 
macro_rules! create_list_field_struct_and_impl{
    ($tip: expr, $class: ident, $count: ident, $count_type: ty, $value: ident, $value_type: ty) => (


#[derive(Clone)]
pub struct $class  {
	$count: $count_type,
	$value: Vec<$value_type>,
}

impl Serialize for $class {

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, Error> {
        let mut seek = self.$count.parse(buf, seek) ? ;
        let count = self.$count.to_u64() as usize;
        self.$value = Vec::new();
        for _ in 0..count {
            let obj: $value_type;
            (obj, seek) = <$value_type>::create(buf, seek) ? ;
            self.$value.push(obj);
        }
        Ok(seek)
    }

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

impl Describe for $class {

   fn describe(&self) -> String {
       "".to_string()
   }

   fn to_json(&self, cnf: &FieldJsonConfig) -> String {
       "".to_string()
   }

   fn from_json(&mut self, _: &String) -> Option<Error> {
       None
   }

}

impl Field for $class {

   // parse function
   fn_field_create_by_new_wrap_return!($class);

   fn new() -> $class {
        $class {
            $count: <$count_type>::from_uint(0),
            $value: Vec::new(),
        }
   }

}


impl $class {

	pub fn count(&self) -> u64 {
		self.$count.to_u64()
	}

	pub fn value(&self) -> &Vec<$value_type> {
		&self.$value
	}

	pub fn append(&mut self, v: $value_type) {
		self.$count += 1u32;
        self.$value.push(v);
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

create_list_field_struct_and_impl!("TEST", Test9375649365, count, Uint1, lists, Bool);

