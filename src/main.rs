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
  let mut layout = Layout::new(View::new(
    style! {
      background-color: { rgba(139, 195, 74, 1.0) };
      justify-content: { space-betweenft7, };
      flex-direction: { row };
      align-items: { center };
    },

    vec![
      View::new(style! {
        background-color: { rgba(255, 255, 255, 1.0) };
        margin-right: { 10 px };
        height: { 300 px };
        width: { 300 px };
      }, vec![]),

      View::new(style! {
        background-color: { rgba(33, 150, 243, 1.0) };
        margin-left: { 10 px };
        height: { 300 px };
        width: { 300 px };
      }, vec![]),

      View::new(style! {
        background-color: { rgba(255, 87, 34, 1.0) };
        justify-content: { space-between };
        flex-direction: { column };
        align-items: { center };
        margin-left: { 20 px };
        height: { 300 px };
        width: { 300 px };
      }, vec![
//        View::new(style! {
//          background-color: { rgba(255, 255, 255, 1.0) };
//          height: { 30 px };
//          width: { 30 px };
//        }, vec![]),
//
//        View::new(style! {
//          background-color: { rgba(33, 150, 243, 1.0) };
//          height: { 30 px };
//          width: { 30 px };
//        }, vec![]),
//
//        View::new(style! {
//          background-color: { rgba(255, 87, 34, 1.0) };
//          height: { 30 px };
//          width: { 30 px };
//        }, vec![])
      ])
    ]
  ));

  let mut app = App::new(layout);
  app.run();
}