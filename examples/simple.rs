#![feature(box_syntax)]
#![feature(proc_macro)]

extern crate rise;
extern crate rise_stylesheet;

use rise::{App, Layout, ViewId, WindowOptions, WindowPosition};
use rise_stylesheet::styles::prelude::{Style, Stylesheet};

fn get_style(stylesheet: &Stylesheet, style_name: &str) -> Style {
  let mut style = stylesheet.take(style_name.to_string()).unwrap();
  style.apply_tag("default".to_string());
  style
}

fn main() {
  let stylesheet = {
    let mut stylesheet = Stylesheet::default();
    stylesheet
      .load_from_string(include_str!("styles.json").to_string())
      .unwrap();

    stylesheet
  };

  let mut layout: Layout<()> = Layout::new();
  let arena = layout.get_arena();

  let root = layout.create_view((), get_style(&stylesheet, "layout"));
  let child = layout.create_view((), get_style(&stylesheet, "circle"));

  root.append(child.clone(), &mut arena.borrow_mut());
  layout.set_root(root);

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
