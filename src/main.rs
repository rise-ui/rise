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

// Layout
use layout::layout::{ Layout, trace_nodes };
use layout::styles::Style;
use layout::view::View;
use app::App;

fn main() {
  let mut stylesheet: Stylesheet = css! {
    .container {
       background-color: rgba(0, 0, 0, 1);
       justify-content: space-between;
       flex-direction: row;
       align-items: center;
    }

    .item {
      background-color: rgba(0, 0, 0, 1);
      height: 300px;
      width: 250px;
    }
  };

  let mut layout = Layout::new(View::new(
    stylesheet.take(".container"),
    vec![
      View::new(stylesheet.take(".item"), vec![]),
      View::new(stylesheet.take(".item"), vec![])
    ]
  ));

  let mut app = App::new(layout);
  app.run();
}