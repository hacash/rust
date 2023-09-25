

// create Fixed macro
#[macro_export] 
macro_rules! create_optional_field_struct_and_impl{
    ($tip:expr, $name:ident, $value_name:ident, $value_type:ident) => (


#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct $name {
    exist: Bool,
    $value_name: Option<$value_type>,
}


impl Serialize for $name {

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, Error> {
        let mut seek = self.exist.parse(buf, seek) ? ;
        if self.exist.check() {
            let (var, mvsk) = <$value_type>::create(buf, seek) ? ;
            self.$value_name = Some(var);
            seek = mvsk
        }
        Ok(seek)
    }

    fn serialize(&self) -> Vec<u8> {
        let mut resdt = self.exist.serialize();
        if self.exist.check() {
            let mut vardt = self.$value_name.as_ref().unwrap().serialize();
            resdt.append(&mut vardt);
        }
        resdt
    }

    fn size(&self) -> usize {
        let mut size = self.exist.size();
        if self.exist.check() {
            size += self.$value_name.as_ref().unwrap().size();
        }
        size
    }

}

impl Describe for $name {

   fn describe(&self) -> String {
       "".to_string()
   }

   fn to_json(&self) -> String {
       "".to_string()
   }

   fn from_json(&mut self, _: &String) -> Option<Error> {
       None
   }

}

impl Field for $name {

   // parse function
   pub_fn_field_create_by_new_wrap_return!($name);

   fn new() -> $name {
        $name {
            exist: Bool::from(false),
            $value_name: None,
        }
   }

}


impl $name {

    pub fn must(v: $value_type) -> $name {
        $name {
            exist: Bool::from(true),
            $value_name: Some(v),
        }
    }

    pub fn from(ifv: Option<$value_type>) -> $name {
        match ifv {
            Some(v) => <$name>::must(v),
            _ => <$name>::new(),
        }
    }

    pub fn is_exist(&self) -> bool {
        self.exist.check()
    }

    pub fn get_value(&self) -> Option<& $value_type> {
        match &self.$value_name {
            Some(v) => Some(&v),
            None => None,
        }
    }
    
    // clone
    pub fn gain_value(&self) -> $value_type {
        match self.exist.check() {
            true => self.$value_name.clone().unwrap(),
            false => <$value_type>::new(),
        }
    }
    

}




    )
}





