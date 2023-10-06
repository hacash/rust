

// create
create_bytes_struct_and_impl!("BytesMaxLen1", BytesMaxLen1, Uint1, 255usize);
create_bytes_struct_and_impl!("BytesMaxLen2", BytesMaxLen2, Uint2, 65535usize);
create_bytes_struct_and_impl!("BytesMaxLen4", BytesMaxLen4, Uint4, 4294967295usize);

