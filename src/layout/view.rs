use rsx_primitives::rsx_stylesheet::types::StyleDeclarations;
use layout::styles::Style;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct View {
  pub childs: Vec<View>,
  pub style: Rc<RefCell<Style>>
}

impl View {
  pub fn new(style: StyleDeclarations, childs: Vec<View>) -> View {
    let style = Style::new(style);
    let style = Rc::new(RefCell::new(style));
    View::prepare(style.clone(), &childs);

    View {
      childs,
      style
    }
  }

  fn prepare(parent_style: Rc<RefCell<Style>>, childs: &Vec<View>) {
    for (index, child) in childs.iter().enumerate() {
      let child = child.clone();
      let index = index as u32;

      let parent_style_node = parent_style.borrow_mut().node.clone();
      let child_node = child.style.borrow_mut().node.clone();

      parent_style_node.borrow_mut().insert_child(&mut *child_node.borrow_mut(), index);
    }
  }
}