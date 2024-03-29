


#[macro_export]
macro_rules! defineChainStateOperationInstance{
    (
        $name:ident
        ($( $kfix1:expr, $name1:ident, $vtype1:ty )+)
        ($( $kfix2:expr, $name2:ident, $keyty2:ty, $vtype2:ty )+)
    ) => (



concat_idents!(struct_name_read = $name, Read {
pub struct struct_name_read<'a> {
    db: &'a dyn StoreDB,
}
impl struct_name_read<'_> {
    pub fn wrap<'a>(sta: &'a dyn StoreDB) -> struct_name_read {
        struct_name_read{
            db: sta,
        }
    }

    // get block_reward
    $(
        concat_idents!(fn_get_1 = $name1 {
        pub fn fn_get_1(&self) -> $vtype1 {
            let mut obj = <$vtype1>::new();
            if (*self.db).load( $kfix1, &Empty::new(), &mut obj) {
                return obj
            }
            $vtype1::new()
        }
        });
    )+
    // get balance
    $(
        concat_idents!(fn_get_2 = $name2 {
        pub fn fn_get_2(&self, $name2: &$keyty2) -> Option<$vtype2> {
            let mut obj = <$vtype2>::new();
            if (*self.db).load($kfix2, $name2, &mut obj) {
                return Some(obj)
            }
            None
        }
        });
    )+


}

});


///////////////

pub struct $name<'a> {
    db: &'a mut dyn StoreDB,
}

impl $name<'_> {

    pub fn wrap(sta: &mut dyn StoreDB) -> $name {
        $name{
            db: sta,
        }
    }

    // get block_reward
    $(
        concat_idents!(fn_get_1 = $name1 {
        pub fn fn_get_1(&self) -> $vtype1 {
            let mut obj = <$vtype1>::new();
            (*self.db).load( $kfix1, &Empty::new(), &mut obj);
            obj
        }
        });
    )+

    // set block_reward
    $(
        concat_idents!(fn_set_1 = set_, $name1 {
        pub fn fn_set_1(&self, obj: &$vtype1) {
            (*self.db).set( $kfix1, &Empty::new(), obj);
        }
        });
    )+

    // put block_reward
    $(
        concat_idents!(fn_put_1 = put_, $name1 {
        pub fn fn_put_1(&mut self, obj: &$vtype1) {
            let mut sta = &mut self.db;
            sta.put( $kfix1, &Empty::new(), obj);
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
        pub fn fn_set_2(&self, key: &$keyty2, obj: &$vtype2) {
            (*self.db).set($kfix2, key, obj);
        }
        });
    )+

    // put balance
    $(
        concat_idents!(fn_put_2 = put_, $name2 {
        pub fn fn_put_2(&mut self, key: &$keyty2, obj: &$vtype2) {
            let mut sta = &mut self.db;
            sta.put($kfix2, key, obj);
        }
        });
    )+


    // del balance
    $(
        concat_idents!(fn_del_2 = del_, $name2 {
        pub fn fn_del_2(&mut self, key: &$keyty2) {
            (*self.db).del($kfix2, key);
        }
        });
    )+

    // rm balance
    $(
        concat_idents!(fn_rm_2 = rm_, $name2 {
        pub fn fn_rm_2(&mut self, key: &$keyty2) {
            let mut sta = &mut self.db;
            sta.rm($kfix2, key);
        }
        });
    )+


}



    )
}


