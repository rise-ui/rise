#![feature(box_syntax)]
#![feature(proc_macro)]

extern crate ordered_float;
extern crate quick_tk;
extern crate rsx_stylesheet;

mod common;

use ordered_float::OrderedFloat;
use rsx_stylesheet::types::Stylesheet;
use rsx_stylesheet::types::*;
use rsx_stylesheet::*;

use quick_tk::{App, Layout, View, WindowOptions, WindowPosition};

fn main() {
  let sidebar_view = View::new(
    style! {
      background-color: { rgb(22, 69, 162) };
      border-bottom-left-radius: { 15 px };
      border-top-left-radius: { 15 px };

      justify-content: { space-between };
      flex-direction: { column };
      align-items: { center };

      padding-bottom: { 30 px };
      padding-top: { 30 px };
      width: { 80 px };
    },
    vec![
      View::new(
        style!{
          justify-content: { center };
          flex-direction: { column };
          align-items: { center };
        },
        vec![
          View::new(
            common::add_border_radius_to_all(
              style! {
                background-color: { rgba(255, 255, 255, 0.75) };
                height: { 50 px }; width: { 50 px };
              },
              50.0,
            ),
            vec![],
          ),
          View::new(
            common::add_border_radius_to_all(
              style! {
                background-color: { rgba(255, 255, 255, 0.5) };
                height: { 40 px }; width: { 40 px };
                margin-top: { 15 px };
              },
              40.0,
            ),
            vec![],
          ),
          View::new(
            common::add_border_radius_to_all(
              style! {
                background-color: { rgba(255, 255, 255, 0.5) };
                height: { 40 px }; width: { 40 px };
                margin-top: { 15 px };
              },
              40.0,
            ),
            vec![],
          ),
        ],
      ),
      View::new(
        style!{
          justify-content: { center };
          flex-direction: { column };
          align-items: { center };
        },
        vec![
          View::new(
            common::add_border_radius_to_all(
              style! {
                background-color: { rgba(255, 255, 255, 0.5) };
                height: { 30 px }; width: { 30 px };
              },
              30.0,
            ),
            vec![],
          ),
          View::new(
            common::add_border_radius_to_all(
              style! {
                background-color: { rgba(255, 255, 255, 0.5) };
                height: { 30 px }; width: { 30 px };
                margin-top: { 15 px };
              },
              30.0,
            ),
            vec![],
          ),
        ],
      ),
    ],
  );

  fn get_sidebar_list() -> Vec<View> {
    let mut list = vec![];

    for _ in 0..5 {
      list.push(View::new(
        style! {
          flex-direction: { row };
          align-items: { center };
          margin-top: { 10 px };
          width: { 100 % };
        },
        vec![
          View::new(
            common::add_border_radius_to_all(
              style! {
                background-color: { rgba(0,0,0, 0.1) };
                margin-right: { 10 px };
                width: { 30 px };
                height: { 30 px };
              },
              30.0,
            ),
            vec![],
          ),
          View::new(
            common::add_border_radius_to_all(
              style! {
                background-color: { rgba(0,0,0, 0.1) };
                height: { 20 px };
                flex: { 1 };
              },
              4.0,
            ),
            vec![],
          ),
        ],
      ));
    }

    return list;
  }

  let sidebar_menu = View::new(
    style!{
      background-color: { rgb(238,238,238) };
      border-bottom-left-radius: { 15 px };
      border-top-left-radius: { 15 px };

      justify-content: { flex-start };
      flex-direction: { column };
      align-items: { center };

      padding-bottom: { 30 px };
      padding-top: { 30 px };
      padding-left: { 15 px };
      padding-right: { 15 px };
      width: { 220 px };
    },
    vec![
      //  Avatar view
      View::new(
        style!{
          flex-direction: { row };
          align-items: { center };
          width: { 100 % };
        },
        vec![
          View::new(
            common::add_border_radius_to_all(
              style! {
                background-color: { rgba(0,0,0, 0.1) };
                margin-right: { 10 px };
                width: { 40 px };
                height: { 40 px };
              },
              8.0,
            ),
            vec![],
          ),
          View::new(
            common::add_border_radius_to_all(
              style! {
                background-color: { rgba(0,0,0, 0.1) };
                height: { 25 px };
                flex: { 1 };
              },
              4.0,
            ),
            vec![],
          ),
        ],
      ),
      // List view
      View::new(
        style!{
          flex-direction: { column };

          padding-right: { 10 px };
          padding-left: { 10 px };
          margin-top: { 15 px };
          width: { 100 % };
        },
        get_sidebar_list(),
      ),
    ],
  );

  let content_view = View::new(
    style! {
      background-color: { rgb(255, 255, 255) };
      border-bottom-right-radius: { 15 px };
      border-top-right-radius: { 15 px };
      flex: { 1 };

      justify-content: { flex-start };
      flex-direction: { column };
      align-items: { center };

      padding-top: { 30 px };
    },
    vec![
      View::new(
        common::add_border_radius_to_all(
          style! {
            background-color: { rgba(0,0,0, 0.1) };
            width: { 300 px };
            height: { 30 px };
          },
          4.0,
        ),
        vec![],
      ),
      View::new(
        common::add_border_radius_to_all(
          style! {
            background-color: { rgba(0,0,0, 0.1) };
            width: { 180 px };
            height: { 20 px };

            margin-top: { 15 px };
          },
          4.0,
        ),
        vec![],
      ),
      // List view
      View::new(
        style!{
          flex-direction: { column };

          padding-right: { 25 px };
          padding-left: { 25 px };
          margin-top: { 45 px };
          width: { 100 % };
        },
        get_sidebar_list(),
      ),
    ],
  );

  let layout = Layout::new(View::new(
    common::add_border_radius_to_all(
      style!{
        background-color: { rgb(255, 255, 255) };
        margin-top   : { 0 px };
        margin-left  : { 0 px };
        margin-bottom: { 0 px };
        margin-right : { 0 px };
        flex-direction: { row };
      },
      15.0,
    ),
    vec![sidebar_view, sidebar_menu, content_view],
  ));

  let app = App::new(
    WindowOptions {
      title: String::from("Example App"),
      position: WindowPosition::Center,
      window_size: (1000, 700),
    },
    layout,
  );
  app.run();
}
