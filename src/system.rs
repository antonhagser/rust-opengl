use crate::context::Context;

pub trait System {
    fn awake(&mut self, _: &mut Context) {}
    fn update(&mut self, _: &mut Context) {}
    fn late_update(&mut self, _: &mut Context) {}
}