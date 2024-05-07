

// create Fixed macro
#[macro_export] 
macro_rules! StructFieldOptional {
    ($class:ident, $value:ident, $value_type:ident) => (


#[derive(Default, Clone, PartialEq, Eq)]
pub struct $class {
    exist: Bool,
    $value: Option<$value_type>,
}

impl std::fmt::Debug for $class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"[ifval]")
    }
}

impl Parse for $class {

    fn parse(&mut self, buf: &[u8], seek: usize) -> Ret<usize> {
        // println!("{}", hex::encode(buf));
        // println!("StructFieldOptional parse exist {} {}", buf.len(), seek);
        let mut seek = self.exist.parse(buf, seek) ?;
        // println!("StructFieldOptional parse {}", seek);
        if self.is_exist() {
            let (val, mvsk) = <$value_type>::create(&buf[seek..]) ?;
            self.$value = Some(val);
            seek += mvsk
        }
        Ok(seek)
    }
}

impl Serialize for $class {

    fn serialize(&self) -> Vec<u8> {
        let mut resdt = self.exist.serialize();
        if self.is_exist() {
            let mut vardt = self.$value.as_ref().unwrap().serialize();
            resdt.append(&mut vardt);
        }
        resdt
    }

    fn size(&self) -> usize {
        let mut size = self.exist.size();
        if self.is_exist() {
            size += self.$value.as_ref().unwrap().size();
        }
        size
    }

}

impl Field for $class {

    // must & create function
    fnFieldMustCreate!($class);
}


impl $class {
    
    pub fn is_exist(&self) -> bool {
        self.exist.check()
    }

    pub fn must(v: $value_type) -> $class {
        $class {
            exist: Bool::from_bool(true),
            $value: Some(v),
        }
    }

    pub fn from_value(ifv: Option<$value_type>) -> $class {
        match ifv {
            Some(v) => <$class>::must(v),
            _ => <$class>::default(),
        }
    }

    pub fn if_value(&self) -> Option<& $value_type> {
        match &self.$value {
            Some(v) => Some(&v),
            None => None,
        }
    }
    
    // clone
    pub fn value(&self) -> $value_type {
        match self.exist.check() {
            true => self.$value.as_ref().unwrap().clone(),
            false => <$value_type>::default(),
        }
    }
    

}




    )
}
