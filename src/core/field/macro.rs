
// parse move seek check
macro_rules! parse_move_seek_or_buf_too_short_error{
    ($tip:expr, $seek:expr, $sk:expr, $buf:expr) => ( {
        let mvseek = $seek + $sk;
        let buflen = $buf.len();
        match mvseek <= buflen {
            true => mvseek,
            false => {
                let n1 = &mvseek.to_string();
                let n2 = &buflen.to_string();
                return Err("field::".to_owned()+$tip+".parse() buf too short, need "+n1+" but got "+n2)
            },
        }
    })
}


macro_rules! parse_field_or_return_err{
    ($tip:expr, $type:ty, $buf:expr, $seek:expr) => ({
        let res = <$type>::create($buf, $seek);
        match res {
            Err(e) => return Err(format!("{}.prase error: {}", $tip, e)),
            Ok(r) => r,
        }
    })
}

// macro 

// impl Add, Sub, Mul, Div for Fixed1
macro_rules! impl_operation_for_common{
    ($class:ident, $operate_name:ident, $operate_fn:ident) => (
        impl $operate_name for $class {
            type Output = Self;
            fn $operate_fn(self, other: Self) -> Self {
                let rv = self.to_u64().$operate_fn(other.to_u64());
                <$class>::from_uint(rv)
            }
        }
    )
}


// impl Add<u32,i32,i8...>, Sub<...>, Mul, Div for Fixed1
macro_rules! impl_operation_for_int{
    ($class:ident, $tarty:ident, $operate_name:ident, $operate_fn:ident) => (
        impl $operate_name<$tarty> for $class {
            type Output = Self;
            fn $operate_fn(self, other: $tarty) -> Self {
                let rv = self.to_u64().$operate_fn(other as u64);
                <$class>::from_uint(rv)
            }
        }
    )
}


// impl Add<f32,f64...>, Sub<...>, Mul, Div for Fixed4 or Fixed8
macro_rules! impl_operation_for_float{
    ($class:ident, $tarty:ident, $operate_name:ident, $operate_fn:ident) => (
        impl $operate_name<$tarty> for $class {
            type Output = Self;
            fn $operate_fn(self, other: $tarty) -> Self {
                let rv = self.to_f64().$operate_fn(other as f64);
                <$class>::from_float(rv)
            }
        }
    )
}


// impl AddAssign<u32,i32,i8...>, SubAssign<...>, MulAssign, DivAssign for Fixed1
macro_rules! impl_operation_assign_for_int{
    ($class:ident, $tarty:ident, $operate_name:ident, $operate_fn:ident, $operate_fn_do:ident) => (
        impl $operate_name<$tarty> for $class {
            fn $operate_fn(&mut self, other: $tarty) {
                let nv = self.to_u64().$operate_fn_do(other as u64);
                self.from_u64(nv);
            }
        }
    )
}
