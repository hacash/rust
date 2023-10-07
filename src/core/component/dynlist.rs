
// dyn obj list


#[macro_export]
macro_rules! create_dyn_obj_list_field_struct_and_impl {
    ($class: ident, $lenty: ty, $dynty: ident, $parseobjfunc: path) => (

pub struct $class {
    count: $lenty,
    vlist: Vec<Box<dyn $dynty>>
}

impl Serialize for $class {

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, Error> {
        let mut seek = self.count.parse(buf, seek) ? ;
        let count = self.count.to_u64() as usize;
        self.vlist = Vec::new();
        for _ in 0..count {
            let(obj, mvsk) = $parseobjfunc(buf, seek) ? ;
            seek = mvsk;
            self.vlist.push(obj);
        }
        Ok(seek)
    }

    fn serialize(&self) -> Vec<u8> {
        let mut bts = vec![];
        let bt1 = self.count.serialize();
        bts.push(bt1);
        for i in 0..self.count.to_u64() as usize {
            let bt = self.vlist[i as usize].as_ref().serialize();
            bts.push(bt);
        }
        bts.concat()
    }

    fn size(&self) -> usize {
        let mut sznum = self.count.size();
        for i in 0..self.count.to_u64() as usize {
            sznum += self.vlist[i as usize].as_ref().size();
        }
        sznum
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
            count: <$lenty>::new(),
            vlist: Vec::new(),
        }
   }

}



    )
}




/////////////////////


// create_dyn_obj_list_field_struct_and_impl!{DynListActionMax65535, Uint2, Action, action::create}


