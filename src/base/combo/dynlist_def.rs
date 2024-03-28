
// dyn obj list


#[macro_export]
macro_rules! StructFieldDynList {
    ($class: ident, $lenty: ty, $dynty: ident, $parseobjfunc: path) => (

pub struct $class {
    count: $lenty,
    vlist: Vec<Box<dyn $dynty>>
}

impl Parse for $class {

    fn parse(&mut self, buf: &[u8], seek: usize) -> Result<usize, Error> {
        let mut seek = self.count.parse(buf, seek) ?;
        let count = self.count.to_u64() as usize;
        self.vlist = Vec::new();
        for _ in 0..count {
            let(obj, mvsk) = $parseobjfunc(&buf[seek..]) ?;
            seek = mvsk;
            self.vlist.push(Box::new(obj));
        }
        Ok(seek)
    }
}

impl Serialize for $class {
    
    fn serialize(&self) -> Vec<u8> {
        let mut bts = vec![];
        let bt1 = self.count.serialize();
        bts.push(bt1);
        for i in 0..self.count.to_usize() {
            let bt = self.vlist[i].as_ref().serialize();
            bts.push(bt);
        }
        bts.concat()
    }

    fn size(&self) -> usize {
        let mut sznum = self.count.size();
        for i in 0..self.count.to_usize() {
            sznum += self.vlist[i as usize].as_ref().size();
        }
        sznum
    }

}

impl Field for $class {

   fn new() -> $class {
        $class {
            count: <$lenty>::new(),
            vlist: Vec::new(),
        }
   }

   // create function
   fnFieldMustCreate!($class);

}

impl $class {

	pub fn push(&mut self, v: Box<dyn $dynty>) {
		self.count += 1u8;
        self.vlist.push(v);
	}

	pub fn pop(&mut self) -> Option<Box<dyn $dynty>> {
        let n = self.count.to_u64();
        match n {
            0 => None,
            _ => {
                self.count -= 1u8;
                self.vlist.pop()
            }
        }
	}
}



    )
}



// test
StructFieldDynList!{DynList278452983475923874, Uint1, Field, Uint1::create}


