use super::{
    hex::HexColor,
    prelude::{Color, RGBAColor},
    Int,
};

#[derive(Debug, Clone, Copy)]
pub struct RGBColor<T>
where
    T: Int<T>,
{
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> RGBColor<T>
where
    T: Int<T>,
{
    pub fn new(r: T, g: T, b: T) -> RGBColor<T> {
        RGBColor { r, g, b }
    }
}

impl<T> Color<T> for RGBColor<T>
where
    T: Int<T>,
{
    type Output = (T, T, T);

    fn hex(&self) -> HexColor<T> {
        let r: i32 = num::cast(self.r).unwrap();
        let g: i32 = num::cast(self.g).unwrap();
        let b: i32 = num::cast(self.b).unwrap();
        let hex = (r << 16) + (g << 8) + b;
        HexColor::new(num::cast(hex).unwrap())
    }

    fn rgb(&self) -> RGBColor<T> {
        self.to_owned()
    }

    fn raw(&self) -> (T, T, T) {
        (self.r, self.g, self.b)
    }

    fn invert(&self) -> RGBColor<T> {
        let inv: T = num::cast(T::get_range().end).unwrap();
        RGBColor::new(inv - self.r, inv - self.g, inv - self.b)
    }

    fn rgba(&self) -> RGBAColor<T> {
        RGBAColor::new(self.r, self.g, self.b, T::get_range().end)
    }
}

impl<T> std::ops::Div<u8> for RGBColor<T>
where
    T: Int<T>,
{
    type Output = RGBColor<f32>;

    fn div(self, rhs: u8) -> Self::Output {
        let rhs: f32 = num::cast(rhs).unwrap();
        let r: f32 = num::cast(self.r).unwrap();
        let g: f32 = num::cast(self.g).unwrap();
        let b: f32 = num::cast(self.b).unwrap();

        RGBColor::new(r / rhs, g / rhs, b / rhs)
    }
}

impl<T> std::ops::Mul<f32> for RGBColor<T>
where
    T: Int<T>,
{
    type Output = RGBColor<u8>;

    fn mul(self, rhs: f32) -> Self::Output {
        let rhs: T = num::cast(rhs).unwrap();
        let r: u8 = num::cast(self.r * rhs).unwrap();
        let g: u8 = num::cast(self.g * rhs).unwrap();
        let b: u8 = num::cast(self.b * rhs).unwrap();

        RGBColor::new(r, g, b)
    }
}
