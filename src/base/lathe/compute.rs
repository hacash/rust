

// impl Add, Sub, Mul, Div for Fixed1
#[macro_export]
macro_rules! implComputeTraitCommon{
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
#[macro_export]
macro_rules! implComputeTraitInt{
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
#[macro_export]
macro_rules! implComputeTraitFloat{
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
#[macro_export]
macro_rules! implComputeAssignTraitInt{
    ($class:ident, $tarty:ident, $operate_name:ident, $operate_fn:ident, $operate_fn_do:ident) => (
        impl $operate_name<$tarty> for $class {
            fn $operate_fn(&mut self, other: $tarty) {
                let nv = self.to_u64().$operate_fn_do(other as u64);
                self.parse_u64(nv);
            }
        }
    )
}
