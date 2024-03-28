

// create Fixed macro
#[macro_export] 
macro_rules! StructFieldOptional {
    ($class:ident, $value:ident, $value_type:ident) => (


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct $class {
    exist: Bool,
    $value: Option<$value_type>,
}


impl Parse for $class {

    fn parse(&mut self, buf: &[u8], seek: usize) -> Result<usize, Error> {
        let mut seek = self.exist.parse(buf, seek) ?;
        if self.is_exist() {
            let (val, mvsk) = <$value_type>::create(&buf[seek..]) ?;
            self.$value = Some(val);
            seek = mvsk
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

    fn new() -> $class {
        $class {
            exist: Bool::from_bool(false),
            $value: None,
        }
    }

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
            _ => <$class>::new(),
        }
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








// test
StructFieldOptional!{ TestOptVal983456298456374, ttt, Uint1 }