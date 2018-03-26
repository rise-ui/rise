use webrender::webrender_api::*;
use layout::styles::Style;
use render::RenderBuilder;
use layout::view::View;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Layout {
  pub root: View
}

impl Layout {
  pub fn new(root: View) -> Layout {
    return Layout {
      root
    }
  }

  pub fn calculate(&mut self, window_size: (f32, f32)) {
    use rsx_primitives::rsx_stylesheet::yoga;

    let root_style = self.root.style.clone();
    let root_node = root_style.borrow_mut().node.clone();

    root_node.borrow_mut().calculate_layout(
      window_size.0, window_size.1,
      yoga::Direction::LTR
    );
  }

  pub fn render(&mut self, builder_context: Rc<RefCell<RenderBuilder>>) {
    Layout::draw(&self.root, builder_context);
  }

  pub fn draw(root: &View, builder_context: Rc<RefCell<RenderBuilder>>) {
    let childs = &root.childs;
    let style = &root.style;

    style.borrow_mut().draw(builder_context.clone());

    for child in childs.iter() {
      Layout::draw(&child, builder_context.clone());
      builder_context.borrow_mut().builder.pop_stacking_context();
    }
  }
}

pub fn trace_nodes(root: &View, offset: usize) {
  let root_style = root.style.clone();
  let root_node = root_style.borrow_mut().node.clone();

  let layout = root_node.borrow_mut().get_layout();
  let mut offset_string = String::new();

  for i in 0..offset { offset_string += " "; }

  let trace = vec![
    format!("{}View: (height: {}, width: {})", offset_string.clone(), layout.height(), layout.width()),
    format!("{}OnLayoutProps: (top: {}, right: {}, bottom: {}, left: {})", offset_string.clone(), layout.top(), layout.right(), layout.bottom(), layout.left())
  ];

  println!("{}", trace.join("\n"));
  for (index, node) in root.childs.iter().enumerate() {
    trace_nodes(node, offset + 2);
  }
}