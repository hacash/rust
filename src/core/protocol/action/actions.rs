
// include!{} and parse match by kind
actions_parse_func_and_include!(

    1,
    5,
    6,
    8,

);

/////////////


// dyn list og Action
create_dyn_obj_list_field_struct_and_impl!{DynListActionMax65535, Uint2, Action, create}

