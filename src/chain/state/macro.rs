


#[macro_export]
macro_rules! defineChainStateOperationInstance{
    (
        $name:ident
        ($( $kfix1:expr, $name1:ident, $vtype1:ty )+)
        ($( $kfix2:expr, $name2:ident, $keyty2:ty, $vtype2:ty )+)
    ) => (

pub struct $name<'a> {
    state: &'a mut dyn State,
}

impl $name<'_> {

    pub fn wrap(sta: &mut dyn State) -> $name {
        $name{
            state: sta,
        }
    }

    // get block_reward
    $(
        concat_idents!(fn_get_1 = $name1 {
        pub fn fn_get_1(&self) -> $vtype1 {
            let mut obj = <$vtype1>::new();
            if (*self.state).load( $kfix1, &Empty::new(), &mut obj) {
                return obj
            }
            $vtype1::new()
        }
        });
    )+

    // set block_reward
    $(
        concat_idents!(fn_set_1 = set_, $name1 {
        pub fn fn_set_1(&mut self, obj: &$vtype1) {
            let mut sta = &mut self.state;
            sta.set( $kfix1, &Empty::new(), obj);
        }
        });
    )+


    // get balance
    $(
        concat_idents!(fn_get_2 = $name2 {
        pub fn fn_get_2(&self, $name2: &$keyty2) -> Option<$vtype2> {
            let mut obj = <$vtype2>::new();
            if (*self.state).load($kfix2, $name2, &mut obj) {
                return Some(obj)
            }
            None
        }
        });
    )+

    // set balance
    $(
        concat_idents!(fn_set_2 = set_, $name2 {
        pub fn fn_set_2(&mut self, key: &$keyty2, obj: &$vtype2) {
            let mut sta = &mut self.state;
            sta.set($kfix2, key, obj);
        }
        });
    )+

    // del balance
    $(
        concat_idents!(fn_del_2 = del_, $name2 {
        pub fn fn_del_2(&mut self, key: &$keyty2) {
            let mut sta = &mut self.state;
            sta.del($kfix2, key);
        }
        });
    )+


}



    )
}


