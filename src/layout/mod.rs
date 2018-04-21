pub mod view;
pub use layout::view::{View, ViewRef};

use render::RenderBuilder;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Layout {
  pub root: ViewRef<()>,
}

impl Layout {
  pub fn new(root: ViewRef<()>) -> Layout {
    return Layout { root };
  }

  pub fn calculate(&mut self, window_size: (f32, f32)) {
    self.root.calculate_layout(window_size);
  }

  pub fn render(&mut self, render: Rc<RefCell<RenderBuilder>>) {
    self.root.render(render.clone());
  }
}
