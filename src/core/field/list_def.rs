

macro_rules! pub_struct_define_for_list{
    ($class: ident, $count: ident, $count_type: ty, $value: ident, $value_type: ty) => (


#[derive(Clone)]
pub struct $class  {
	$count: $count_type,
	$value: Vec<$value_type>,
}


	)
}