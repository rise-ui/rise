#![feature(box_syntax)]
#![feature(proc_macro)]

#[macro_use] extern crate rsx_stylesheet;
#[macro_use] extern crate rsx_shared;

extern crate webrender;
extern crate palette;

extern crate gleam;
extern crate glutin;

extern crate ordered_float;
extern crate env_logger;
extern crate app_units;
extern crate font_loader;
extern crate rusttype;
extern crate failure;
extern crate euclid;

mod geometry;
mod layout;
mod window;
mod render;
mod app;

pub use layout::prelude::*;
pub use window::*;
pub use render::*;
pub use app::*;