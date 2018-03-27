#![feature(box_syntax)]
#![feature(proc_macro)]

extern crate rsx_stylesheet;
extern crate ordered_float;
extern crate quick_tk;

use rsx_stylesheet::types::Stylesheet;
use rsx_stylesheet::types::*;
use rsx_stylesheet::*;
use ordered_float::OrderedFloat;

use quick_tk::{ Layout, View, App };

// HardFix, current `style!` macros not support global `border-radius` property, bot support radius by corner
fn add_border_radius_to_all(mut style: StyleDeclarations, border_radius: f32) -> StyleDeclarations {
  style.push(StyleDeclaration::Theme(ThemeStyle::BorderTopLeftRadius(StyleUnit::Point(OrderedFloat(border_radius)))));
  style.push(StyleDeclaration::Theme(ThemeStyle::BorderTopRightRadius(StyleUnit::Point(OrderedFloat(border_radius)))));
  style.push(StyleDeclaration::Theme(ThemeStyle::BorderBottomRightRadius(StyleUnit::Point(OrderedFloat(border_radius)))));
  style.push(StyleDeclaration::Theme(ThemeStyle::BorderBottomLeftRadius(StyleUnit::Point(OrderedFloat(border_radius)))));

  return style;
}

fn main() {
  let layout = Layout::new(View::new(
    add_border_radius_to_all(style! {
      background-color: { rgb(238,238,238) };
      justify-content: { center };
      flex-direction: { row };
      align-items: { center };
    }, 20.0),

    vec![
      View::new(add_border_radius_to_all(style! {
        background-color: { rgb(3,169,244) };
        margin-right: { 10 px };
        height: { 300 px };
        width: { 300 px };
      }, 30.0), vec![]),

      View::new(style! {
        background-color: { rgb(139,195,74) };
        margin-left: { 10 px };
        height: { 300 px };
        width: { 300 px };

        border-top-left-radius: { 10 px };
        border-bottom-right-radius: { 10 px };
        border-top-right-radius: { 100 px };
        border-bottom-left-radius: { 100 px };
      }, vec![]),

      View::new(style! {
        background-color: { rgb(63,81,181) };
        justify-content: { space-between };
        flex-direction: { column };
        align-items: { center };
        margin-left: { 20 px };
        height: { 300 px };
        width: { 300 px };

        padding-bottom: { 20 px };
        padding-top: { 20 px };

        border-top-left-radius: { 100 px };
        border-bottom-right-radius: { 100 px };
        border-top-right-radius: { 10 px };
        border-bottom-left-radius: { 10 px };
      }, vec![
        View::new(add_border_radius_to_all(style! {
          background-color: { rgb(245,245,245) };
          height: { 50 px };
          width: { 50 px };
        }, 50.0), vec![]),

        View::new(add_border_radius_to_all(style! {
          background-color: { rgb(245,245,245) };
          height: { 50 px };
          width: { 50 px };
        }, 50.0), vec![]),

        View::new(add_border_radius_to_all(style! {
          background-color: { rgb(245,245,245) };
          height: { 50 px };
          width: { 50 px };
        }, 50.0), vec![])
      ])
    ]
  ));

  let app = App::new("Example App", layout);
  app.run();
}