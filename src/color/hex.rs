use super::{
    prelude::{Color, RGBAColor},
    rgb::RGBColor,
    Int,
};

#[derive(Debug, Clone, Copy)]
pub struct HexColor<T = u8>
where
    T: Int<T>,
{
    value: u32,
    x: T,
}

impl<T> HexColor<T>
where
    T: Int<T>,
{
    pub fn new(value: u32) -> Self {
        HexColor { value, x: num::cast(2).unwrap() }
    }
}

impl<T> From<String> for HexColor<T>
where
    T: Int<T>,
{
    fn from(_: String) -> Self {
        todo!()
    }
}

impl<T> Color<T> for HexColor<T>
where
    T: Int<T>,
{
    type Output = u32;

    fn hex(&self) -> HexColor<T> {
        self.to_owned()
    }

    fn rgb(&self) -> RGBColor<T> {
        let val: isize = num::cast(self.value).unwrap();
        let mut r: T = num::cast((val >> 16) & 255).unwrap();
        let mut g: T = num::cast((val >> 8) & 255).unwrap();
        let mut b: T = num::cast(val & 255).unwrap();
        if num::cast::<T, u8>(T::get_range().end).unwrap() == 1 {
            r = r / num::cast(255).unwrap();
            g = g / num::cast(255).unwrap();
            b = b / num::cast(255).unwrap();
        }

        RGBColor::new(r, g, b)
    }

    fn rgba(&self) -> RGBAColor<T> {
        self.rgb().rgba()
    }

    fn raw(&self) -> u32 {
        self.value
    }

    fn invert(&self) -> HexColor<T> {
        self.rgb().invert().hex()
    }
}
