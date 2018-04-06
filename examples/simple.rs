#![feature(box_syntax)]
#![feature(proc_macro)]

extern crate ordered_float;
extern crate rise;
extern crate rsx_stylesheet;

mod common;

use ordered_float::OrderedFloat;
use rsx_stylesheet::types::Stylesheet;
use rsx_stylesheet::types::*;
use rsx_stylesheet::*;

use rise::{App, Layout, View, WindowOptions, WindowPosition};

fn main() {
  let layout = Layout::new(View::new(
    common::add_border_radius_to_all(
      style!{
        background-color: { rgb(224, 224, 224) };
        justify-content: { center };
        flex-direction: { row };
        align-items: { center };

        padding-right: { 30 px };
        padding-left: { 30 px };

        margin-top   : { 0 px };
        margin-left  : { 0 px };
        margin-bottom: { 0 px };
        margin-right : { 0 px };
      },
      15.0,
    ),
    vec![
      View::new(
        common::add_border_radius_to_all(
          style! {
            background-color: { rgb(255, 255, 255) };
            height: { 250 px }; width: { 250 px };
            justify-content: { center };
            flex-direction: { column };
            align-items: { center };

            padding-bottom: { 30 px };
            padding-right: { 30 px };
            padding-left: { 30 px };
            padding-top: { 30 px };
          },
          6.0,
        ),
        vec![
          View::new(
            common::add_border_radius_to_all(
              style! {
                background-color: { rgb(96, 125, 139) };
                height: { 50 px }; width: { 50 px };
              },
              50.0,
            ),
            vec![],
          ),
        ],
      ),
    ],
  ));

  let app = App::new(
    WindowOptions {
      title: String::from("Example App"),
      position: WindowPosition::Center,
      window_size: (500, 500),
    },
    layout,
  );

  app.run();
}
