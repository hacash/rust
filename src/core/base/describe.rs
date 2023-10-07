



#[macro_export]
macro_rules! impl_Describe_trait_for_combine_class {

    ($class: ident, $( $child: ident ),+) => (

        impl Describe for $class {

            fn describe(&self) -> String {
                "".to_string()
            }

            fn to_json(&self, cnf: &FieldJsonConfig) -> String {
                vec![
                $(
                    format!("\"{}\":{}", stringify!($child), self.$child.to_json(cnf)),
                )*
                ].join(",")
            }

            fn from_json(&mut self, _: &String) -> Option<Error> {
                None
            }

        }
    )
}