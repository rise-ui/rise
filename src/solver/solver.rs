use layout::Layout;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solver<T> {
  layout: Rc<RefCell<Layout<T>>>,
  cursor_position: (f64, f64),
  // offset bounds
}

impl<T> Solver<T> {
  pub fn new(layout: Rc<RefCell<Layout<T>>>) -> Solver<T> {
    Solver {
      cursor_position: (0.0, 0.0),
      layout,
    }
  }

  fn calculate(&mut self) {}

  pub fn set_cursor_position(&mut self, position: (f64, f64)) {
    self.cursor_position = position;
    self.calculate();
  }
}
