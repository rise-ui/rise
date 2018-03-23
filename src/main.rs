#![feature(box_syntax)]
#![feature(proc_macro)]

extern crate rsx_primitives;
#[macro_use] extern crate rsx;

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

pub mod geometry;
pub mod layout;
pub mod window;
pub mod render;
pub mod app;

use render::WebRenderContext;
use window::Window;

// RSX
use rsx_primitives::rsx_stylesheet::types::Stylesheet;
use rsx::{rsx, css};

use rsx_primitives::rsx_stylesheet::types::*;
use rsx_primitives::rsx_dom::types::*;
use rsx_primitives::rsx_stylesheet::*;

// Layout
use layout::layout::{ Layout, trace_nodes };
use layout::styles::Style;
use layout::view::View;
use app::App;

fn main() {

  let item_style = style! {
    background-color: { rgb(0, 0, 0) };
    height: { 300 px };
    width: { 250 px };
  };

  let container_style = style! {
    background-color: { rgba(255, 255, 255, 0.8) };
    justify-content: { center };
    flex-direction: { column };
    align-items: { center };
  };

  let mut layout = Layout::new(View::new(
    container_style,
    vec![
      View::new(item_style.clone(), vec![]),
      View::new(item_style.clone(), vec![])
    ]
  ));

  let mut app = App::new(layout);
  app.run();
}