

// create Fixed macro
#[macro_export] 
macro_rules! StructFieldRevMarkEnum {
    ($class:ident, $v1:ident, $t1:ty, $v2:ident, $t2:ty, $swtv: expr) => (


#[derive(Clone, PartialEq, Eq)]
pub enum $class {
    $v1($t1),
    $v2($t2),
}

impl std::fmt::Debug for $class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"[enum]")
    }
}

impl Default for $class {
    fn default() -> Self { Self::$v1(<$t1>::default()) }
}


impl FieldReadable for $class {
    
    fn readable(&self) -> String {
        match self {
            Self::$v1(v) => v.readable(),
            Self::$v2(v) => v.readable(),
        }
    }
    
}


impl Parse for $class {

    fn parse(&mut self, buf: &[u8], seek: usize) -> Ret<usize> {
        if buf.len() < 1 {
            return Err("buf too short".to_owned())
        }
        if buf[0] < $swtv {
            let (v, sk) = <$t1>::create(&buf[seek..]) ? ;
            *self = Self::$v1( v );
            Ok(seek + sk)
        }else{
            let mut nbuf = buf[seek..].to_vec();
            nbuf[0] -= $swtv;
            let (v, sk) = <$t2>::create(&nbuf) ? ;
            *self = Self::$v2( v );
            Ok(seek + sk)
        }
    }
}

impl Serialize for $class {

    fn serialize(&self) -> Vec<u8> {
        match self {
            Self::$v1(v1) => v1.serialize(),
            Self::$v2(v2) => {
                let mut b = v2.serialize();
                let mxv = b[0] as usize + $swtv as usize;
                if mxv > 255 {
                    panic!("mark value too big")
                }
                b[0] += $swtv;
                b
            },
        }
    }

    fn size(&self) -> usize {
        match self {
            Self::$v1(v1) => v1.size(),
            Self::$v2(v2) => v2.size(),
        }
    }

}

impl Field for $class {

    // must & create function
    fnFieldMustCreate!($class);
}


impl $class {
    
    /*
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
    */
    

}




    )
}
