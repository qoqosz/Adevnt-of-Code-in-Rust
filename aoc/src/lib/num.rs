use std::{
    fmt::{Debug, Display},
    u128,
};

pub trait Unsigned: Display + Debug + Copy + Default {}
impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for u128 {}
impl Unsigned for usize {}

pub trait Integer: Display + Debug + Copy + Default {}
impl Integer for u8 {}
impl Integer for i8 {}
impl Integer for u16 {}
impl Integer for i16 {}
impl Integer for u32 {}
impl Integer for i32 {}
impl Integer for u64 {}
impl Integer for i64 {}
impl Integer for u128 {}
impl Integer for i128 {}
impl Integer for usize {}
impl Integer for isize {}
