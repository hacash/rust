

macro_rules! define_action_kind_class_parse_func{
    ( $kindv:expr, $class:ty, { $( $k: ident: $ty:ty )+ }) => (

        // kind define
        concat_idents!(act_kind_value = ACTION_KIND_, $kindv {
            
        pub const act_kind_value: u16 = $kindv;
        
        });

        concat_idents!(act_kind_class_name =  Action, $kindv, $class {

        pub struct act_kind_class_name {
            kind: Uint2,
            $(
                $k: $ty
            ),*
        }

        impl Serialize for act_kind_class_name {

            fn serialize(&self) -> Vec<u8> {
                vec![
                    self.kind.serialize(),
                $(
                    self.$k.serialize(),
                )*
                ].concat()
            }

            fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, Error> {
                let mut sk: usize = seek;
                sk = self.kind.parse(buf, sk) ? ;
                $(
                    sk = self.$k.parse(buf, sk) ? ;
                )*
                Ok(sk)
            }

            fn size(&self) -> usize {
                let mut size: usize = 0;
                size += self.kind.size();
                $(
                    size += self.$k.size();
                )*
                size
            }

        }


        impl Describe for act_kind_class_name {

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



        impl act_kind_class_name {
            const fn kind() -> u16 {
                $kindv
            }

            pub fn get_kind(&self) -> u16 {
                self.kind.to_u16()
            }

            // create function
            pub_fn_field_create_by_new_wrap_return!(Amount);

        }

        });

        // include mod files
        // $(
        //     include!{"
                
        //             concat_idents!(act_kind_class_name = Action, $kindv, $class, .rs {
        //                 act_kind_class_name
        //             });
                
        //     "};
        // )+

        /*
        // parse func
        pub fn parse(buf: &Vec<u8>, seek: usize) -> Result<(Box<dyn Action>, usize), String> {
            let (kindv, _) = parse_move_seek_or_return_err!("actions.parse", Uint2, buf, seek);
            let kdv = kindv.value() as u16;
            match kdv {
            $(
                act_kind_name => {
                    let (act, mvsk) = <$class>::parse(buf, seek) ? ;
                    Ok((Box::new(act), mvsk))
                },
            )+
                _ => Err(format!("Action kind <{}> not find.", kdv))
            }
        }
        */

    )
}


//

