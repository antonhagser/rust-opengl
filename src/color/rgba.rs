use super::{
    hex::HexColor,
    prelude::{Color, RGBColor},
    Int,
};

#[derive(Debug, Clone, Copy)]
pub struct RGBAColor<T>
where
    T: Int<T>,
{
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> RGBAColor<T>
where
    T: Int<T>,
{
    pub fn new(r: T, g: T, b: T, a: T) -> RGBAColor<T> {
        RGBAColor { r, g, b, a }
    }

    pub fn alpha(&mut self, val: T) {
        self.a = val;
    }
}

impl<T> Color<T> for RGBAColor<T>
where
    T: Int<T>,
{
    type Output = (T, T, T, T);

    fn hex(&self) -> HexColor<T> {
        println!("Warn hex: {:?}", T::get_range());
        self.rgb().hex()
    }

    fn rgb(&self) -> RGBColor<T> {
        RGBColor::new(self.r, self.g, self.b)
    }

    fn raw(&self) -> Self::Output {
        (self.r, self.g, self.b, self.a)
    }

    fn invert(&self) -> Self {
        let inv: T = num::cast(T::get_range().end).unwrap();
        RGBAColor::new(inv - self.r, inv - self.g, inv - self.b, self.a)
    }

    fn rgba(&self) -> RGBAColor<T> {
        self.to_owned()
    }
}

impl<T> std::ops::Div<u8> for RGBAColor<T>
where
    T: Int<T>,
{
    type Output = RGBAColor<f32>;

    fn div(self, rhs: u8) -> Self::Output {
        let rhs: f32 = num::cast(rhs).unwrap();
        let r: f32 = num::cast(self.r).unwrap();
        let g: f32 = num::cast(self.g).unwrap();
        let b: f32 = num::cast(self.b).unwrap();
        let a: f32 = num::cast(self.a).unwrap();

        RGBAColor::new(r / rhs, g / rhs, b / rhs, a / rhs)
    }
}

impl<T> std::ops::Mul<f32> for RGBAColor<T>
where
    T: Int<T>,
{
    type Output = RGBAColor<u8>;

    fn mul(self, rhs: f32) -> Self::Output {
        let rhs: T = num::cast(rhs).unwrap();
        let r: u8 = num::cast(self.r * rhs).unwrap();
        let g: u8 = num::cast(self.g * rhs).unwrap();
        let b: u8 = num::cast(self.b * rhs).unwrap();
        let a: u8 = num::cast(self.a * rhs).unwrap();

        RGBAColor::new(r, g, b, a)
    }
}
