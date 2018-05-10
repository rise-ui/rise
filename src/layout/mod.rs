pub mod context;

pub use context::*;
use render::RenderBuilder;
use rise_stylesheet::styles::style::{Style, StyleExt};
use rise_stylesheet::yoga::{Direction, Node};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Layout {
  pub root: Node,
}

impl Layout {
  pub fn new(root: Node) -> Layout {
    return Layout {
      root,
    };
  }

  pub fn calculate(&mut self, window_size: (f32, f32)) {
    self.root.calculate_layout(window_size.0, window_size.1, Direction::LTR);
  }

  pub fn render(&mut self, render: Rc<RefCell<RenderBuilder>>) {
    self.root.render(render);
  }
}
