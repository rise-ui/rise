use rsx_stylesheet::types::StyleDeclarations;
use rsx_stylesheet::yoga::Node;

use render::RenderBuilder;
use std::cell::RefCell;
use std::rc::Rc;

pub trait DrawerStyle {
  fn draw(&mut self, builder_context: Rc<RefCell<RenderBuilder>>);
  fn get_styles(&self) -> StyleDeclarations;
  fn get_node(&self) -> Rc<RefCell<Node>>;
}
