use num::NumCast;
use std::{fmt::{Debug, Display}, ops::{Add, Div, Mul, Sub}};

pub mod hex;
pub mod rgb;
pub mod rgba;

pub trait Int<T>:
    Add<Output = Self>
    + Sub<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + PartialEq
    + Copy
    + NumCast
    + Display
    + Debug
{
    fn get_range() -> std::ops::Range<T>;
}

impl Int<u8> for u8 {
    fn get_range() -> std::ops::Range<u8> {
        0..255
    }
}

impl Int<u32> for u32 {
    fn get_range() -> std::ops::Range<u32> {
        0..255
    }
}

impl Int<i32> for i32 {
    fn get_range() -> std::ops::Range<i32> {
        0..255
    }
}

impl Int<f32> for f32 {
    fn get_range() -> std::ops::Range<f32> {
        0f32..1f32
    }
}

impl Int<f64> for f64 {
    fn get_range() -> std::ops::Range<f64> {
        0f64..1f64
    }
}

pub mod prelude {
    use super::Int;
    pub use super::{hex::HexColor, rgb::RGBColor, rgba::RGBAColor};

    pub trait Color<T>
    where
        T: Int<T>,
    {
        type Output;

        fn hex(&self) -> HexColor<T>;
        fn rgb(&self) -> RGBColor<T>;
        fn rgba(&self) -> RGBAColor<T>;

        fn raw(&self) -> Self::Output;

        fn invert(&self) -> Self;
    }
}
