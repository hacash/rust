

// create Fixed macro
#[macro_export] 
macro_rules! create_optional_field_struct_and_impl{
    ($tip:expr, $class:ident, $value:ident, $value_type:ident) => (


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct $class {
    exist: Bool,
    $value: Option<$value_type>,
}


impl Serialize for $class {

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, Error> {
        let mut seek = self.exist.parse(buf, seek) ? ;
        if self.exist.check() {
            let (var, mvsk) = <$value_type>::create(buf, seek) ? ;
            self.$value = Some(var);
            seek = mvsk
        }
        Ok(seek)
    }

    fn serialize(&self) -> Vec<u8> {
        let mut resdt = self.exist.serialize();
        if self.exist.check() {
            let mut vardt = self.$value.as_ref().unwrap().serialize();
            resdt.append(&mut vardt);
        }
        resdt
    }

    fn size(&self) -> usize {
        let mut size = self.exist.size();
        if self.exist.check() {
            size += self.$value.as_ref().unwrap().size();
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
            exist: Bool::from(false),
            $value: None,
        }
   }

}


impl $class {

    pub fn must(v: $value_type) -> $class {
        $class {
            exist: Bool::from(true),
            $value: Some(v),
        }
    }

    pub fn from(ifv: Option<$value_type>) -> $class {
        match ifv {
            Some(v) => <$class>::must(v),
            _ => <$class>::new(),
        }
    }

    pub fn is_exist(&self) -> bool {
        self.exist.check()
    }

    pub fn get_value(&self) -> Option<& $value_type> {
        match &self.$value {
            Some(v) => Some(&v),
            None => None,
        }
    }
    
    // clone
    pub fn gain_value(&self) -> $value_type {
        match self.exist.check() {
            true => self.$value.clone().unwrap(),
            false => <$value_type>::new(),
        }
    }
    

}




    )
}





