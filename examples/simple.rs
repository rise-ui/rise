#![feature(box_syntax)]
#![feature(proc_macro)]

extern crate rise;
extern crate rise_stylesheet;

use rise::{App, Layout, ViewRef, WindowOptions, WindowPosition};
use rise_stylesheet::styles::prelude::{Style, Stylesheet};

fn get_view_by_style(stylesheet: Stylesheet, style_name: &str) -> ViewRef<()> {
  let mut style = stylesheet.take(style_name.to_string()).unwrap();
  style.apply_tag("default".to_string());
  ViewRef::new((), style)
}

fn main() {
  let stylesheet = {
    let mut stylesheet = Stylesheet::default();
    stylesheet
      .load_from_string(include_str!("styles.json").to_string())
      .unwrap();

    stylesheet
  };

  let layout_container = get_view_by_style(stylesheet.clone(), "layout");
  let circle_child = get_view_by_style(stylesheet.clone(), "circle");
  layout_container.append(circle_child);

  let app = App::new(
    WindowOptions {
      title: String::from("Example App"),
      position: WindowPosition::Center,
      window_size: (500, 500),
    },
    Layout::new(layout_container),
  );

  app.run();
}
