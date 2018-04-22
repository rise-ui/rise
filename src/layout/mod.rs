pub mod view;
pub use layout::view::{Arena, View, ViewId};
use rise_stylesheet::styles::style::Style;

use render::RenderBuilder;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Layout<T> {
  arena: Rc<RefCell<Arena<T>>>,
  root: Option<ViewId>,
}

impl<T> Layout<T> {
  pub fn new() -> Layout<T> {
    return Layout {
      arena: Rc::new(RefCell::new(Arena::new())),
      root: None,
    };
  }

  pub fn calculate(&mut self, window_size: (f32, f32)) {
    if let Some(root_id) = self.root {
      use std::ops::Index;

      let arena = self.arena.clone();
      let arena_borrowed = arena.borrow();

      let root = arena_borrowed.index(root_id.clone()).clone();
      root.calculate_layout(window_size);
    }
  }

  pub fn set_root(&mut self, root_id: ViewId) {
    self.root = Some(root_id)
  }

  pub fn get_arena(&self) -> Rc<RefCell<Arena<T>>> {
    self.arena.clone()
  }

  pub fn render(&mut self, render: Rc<RefCell<RenderBuilder>>) {
    let arena = self.arena.clone();

    if let Some(root_id) = self.root {
      root_id.render(&arena.borrow(), render.clone());
    }
  }

  pub fn create_view(&self, data: T, style: Style) -> ViewId {
    let arena = self.arena.clone();
    let mut arena_borrowed = arena.borrow_mut();
    arena_borrowed.new_node(data, style)
  }
}
