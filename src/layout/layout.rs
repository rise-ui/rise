use layout::view::{DrawerView, View};
use render::RenderBuilder;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Layout {
  pub root: Rc<RefCell<DrawerView>>,
}

impl Layout {
  pub fn new(root: View) -> Layout {
    let root = Rc::new(RefCell::new(root));
    return Layout { root };
  }

  pub fn calculate(&mut self, window_size: (f32, f32)) {
    use rsx_stylesheet::yoga;

    let root_style = self.root.borrow().get_style();
    let root_node = root_style.borrow_mut().get_node();

    root_node
      .borrow_mut()
      .calculate_layout(window_size.0, window_size.1, yoga::Direction::LTR);
  }

  pub fn render(&mut self, render: Rc<RefCell<RenderBuilder>>) {
    self.root.borrow_mut().draw(render.clone());
  }
}
