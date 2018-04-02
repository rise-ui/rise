use layout::style::prelude::{DrawerStyle, Style};
use rsx_stylesheet::types::StyleDeclarations;

use std::boxed::Box;
use std::cell::RefCell;
use std::rc::Rc;

pub struct View {
  pub childs: Vec<View>,
  pub style: Rc<RefCell<DrawerStyle>>,
}

impl View {
  pub fn new(style: StyleDeclarations, childs: Vec<View>) -> View {
    let style = Rc::new(RefCell::new(Style::new(style)));
    View::prepare(style.clone(), &childs);

    View { childs, style }
  }

  fn prepare(parent_style: Rc<RefCell<Style>>, childs: &Vec<View>) {
    for (index, child) in childs.iter().enumerate() {
      let child = child.clone();
      let index = index as u32;

      let parent_style_node = parent_style.borrow_mut().get_node();
      let child_node = child.style.borrow_mut().get_node();

      parent_style_node
        .borrow_mut()
        .insert_child(&mut *child_node.borrow_mut(), index);
    }
  }
}
