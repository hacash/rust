
fn field_parse_uint<U, T>(sf: &mut U, nt: T, sz: usize) -> RetErr
    where U: FieldUint, T: std::ops::Add<u64, Output = u64>
{ 
    let num: u64 = nt + 0u64;
    let err = Err(s!("from_uint size error or cannot over 8"));
    match sz {
        1 => if num <= UINT_MAX_W1 { sf.parse_u8(  num as u8  ) }else{ err },
        2 => if num <= UINT_MAX_W2 { sf.parse_u16( num as u16 ) }else{ err },
        3 => if num <= UINT_MAX_W3 { sf.parse_u32( num as u32 ) }else{ err },
        4 => if num <= UINT_MAX_W4 { sf.parse_u32( num as u32 ) }else{ err },
        5 => if num <= UINT_MAX_W5 { sf.parse_u64( num as u64 ) }else{ err },
        6 => if num <= UINT_MAX_W6 { sf.parse_u64( num as u64 ) }else{ err },
        7 => if num <= UINT_MAX_W7 { sf.parse_u64( num as u64 ) }else{ err },
        8 => if num <= UINT_MAX_W8 { sf.parse_u64( num as u64 ) }else{ err },
        _ => err,
    }
}


fn field_parse_float<U, T>(sf: &mut U, nt: T, sz: usize) -> RetErr 
    where U: FieldFloat, T: std::ops::Add<f64, Output = f64>
{
    let num: f64 = nt + 0f64;
    match sz {
        4 => sf.parse_f32(num as f32),
        8 => sf.parse_f64(num as f64),
        _ => Err(s!("from_float size error must be 4 or 8")),
    }
}



