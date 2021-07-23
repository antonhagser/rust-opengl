#![allow(dead_code)]

#![feature(get_mut_unchecked)]
#![feature(destructuring_assignment)]

#[macro_use]
extern crate log;

mod app;
mod system;
mod engine;
mod context;
mod renderer;

pub mod assets;
pub mod color;

pub use app::App;
pub use system::System;
pub use context::Context;