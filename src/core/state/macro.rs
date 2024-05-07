


#[macro_export]
macro_rules! defineChainStateOperationInstance{
    (
        $base:ident,
        $name:ident,
        ($( $kfix1:expr, $name1:ident, $vtype1:ty )+)
        ($( $kfix2:expr, $name2:ident, $keyty2:ty, $vtype2:ty )+)
    ) => (



concat_idents!(struct_name_disk = $name, Disk {
pub struct struct_name_disk<'a> {
    db: &'a dyn $base,
}
impl struct_name_disk<'_> {
    pub fn wrap<'a>(sta: &'a dyn $base) -> struct_name_disk {
        struct_name_disk{
            db: sta,
        }
    }

    // get block_reward
    $(
        concat_idents!(fn_get_1 = $name1 {
        pub fn fn_get_1(&self) -> $vtype1 {
            let mut obj = <$vtype1>::default();
            if (*self.db).load( $kfix1, &Empty::default(), &mut obj) {
                return obj
            }
            $vtype1::default()
        }
        });
    )+

    // put
    $(
        concat_idents!(fn_put_1 = put_, $name1 {
        pub fn fn_put_1(&self, obj: &$vtype1) {
            (*self.db).put( $kfix1, &Empty::default(), obj);
        }
        });
    )+


    // get balance
    $(
        concat_idents!(fn_get_2 = $name2 {
        pub fn fn_get_2(&self, $name2: &$keyty2) -> Option<$vtype2> {
            let mut obj = <$vtype2>::default();
            if (*self.db).load($kfix2, $name2, &mut obj) {
                return Some(obj)
            }
            None
        }
        });
    )+

    // put
    $(
        concat_idents!(fn_put_2 = put_, $name2 {
        pub fn fn_put_2(&self, key: &$keyty2, obj: &$vtype2) {
            (*self.db).put( $kfix2, key, obj);
        }
        });
    )+

    // rm
    $(
        concat_idents!(fn_rm_2 = rm_, $name2 {
        pub fn fn_rm_2(&mut self, key: &$keyty2) {
            (*self.db).rm( $kfix2, key);
        }
        });
    )+


}

});


///////////////

pub struct $name<'a> {
    db: &'a mut dyn $base,
}

impl $name<'_> {

    pub fn wrap(sta: &mut dyn $base) -> $name {
        $name{
            db: sta,
        }
    }

    // get block_reward
    $(
        concat_idents!(fn_get_1 = $name1 {
        pub fn fn_get_1(&self) -> $vtype1 {
            let mut obj = <$vtype1>::default();
            (*self.db).load( $kfix1, &Empty::default(), &mut obj);
            obj
        }
        });
    )+

    // set block_reward
    $(
        concat_idents!(fn_set_1 = set_, $name1 {
        pub fn fn_set_1(&mut self, obj: &$vtype1) {
            let mut sta = &mut self.db;
            sta.set( $kfix1, &Empty::default(), obj);
        }
        });
    )+

    // put block_reward
    $(
        concat_idents!(fn_put_1 = put_, $name1 {
        pub fn fn_put_1(&self, obj: &$vtype1) {
            (*self.db).put( $kfix1, &Empty::default(), obj);
        }
        });
    )+


    // get balance
    $(
        concat_idents!(fn_get_2 = $name2 {
        pub fn fn_get_2(&self, $name2: &$keyty2) -> Option<$vtype2> {
            let res = (*self.db).get($kfix2, $name2);
            match res {
                Some(dt) => Some(<$vtype2>::must(&dt)), // maybe panic
                _ => None, // not find
            }
        }
        });
    )+

    // set balance
    $(
        concat_idents!(fn_set_2 = set_, $name2 {
        pub fn fn_set_2(&mut self, key: &$keyty2, obj: &$vtype2) {
            let mut sta = &mut self.db;
            sta.set($kfix2, key, obj);
        }
        });
    )+

    // del balance
    $(
        concat_idents!(fn_del_2 = del_, $name2 {
        pub fn fn_del_2(&mut self, key: &$keyty2) {
            let mut sta = &mut self.db;
            sta.del($kfix2, key);
        }
        });
    )+

    // put balance
    $(
        concat_idents!(fn_put_2 = put_, $name2 {
        pub fn fn_put_2(&self, key: &$keyty2, obj: &$vtype2) {
            (*self.db).put($kfix2, key, obj);
        }
        });
    )+

    // rm balance
    $(
        concat_idents!(fn_rm_2 = rm_, $name2 {
        pub fn fn_rm_2(&self, key: &$keyty2) {
            (*self.db).rm($kfix2, key);
        }
        });
    )+


}



    )
}


