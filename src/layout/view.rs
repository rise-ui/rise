use layout::style::prelude::{DrawerStyle, RefCellDrawerStyle, Style};
use rsx_stylesheet::types::StyleDeclarations;

use render::RenderBuilder;
use std::cell::RefCell;
use std::rc::Rc;

fn prepare_node(parent_style: Rc<RefCell<Style>>, childs: &Vec<View>) {
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

pub type RefCellDrawerView = Rc<RefCell<DrawerView>>;

pub trait DrawerView {
  fn draw(&self, render: Rc<RefCell<RenderBuilder>>);
  fn get_parent(&self) -> Option<RefCellDrawerView>;
  fn get_childs(&self) -> &Vec<RefCellDrawerView>;
  fn get_style(&self) -> RefCellDrawerStyle;

  fn set_depth(&mut self, depth: Option<i32>);
  fn get_depth(&self) -> Option<i32> {
    return None;
  }
}

#[derive(Clone)]
pub struct View {
  parent: Option<RefCellDrawerView>,
  childs: Vec<RefCellDrawerView>,
  style: RefCellDrawerStyle,
  depth: Option<i32>,
}

impl DrawerView for View {
  fn draw(&self, render: Rc<RefCell<RenderBuilder>>) {
    let childs = self.get_childs();
    let style = self.get_style();

    style.borrow_mut().draw(render.clone());

    for child in childs.iter() {
      child.borrow_mut().draw(render.clone());
      render.borrow_mut().builder.pop_stacking_context();
    }
  }

  fn get_style(&self) -> RefCellDrawerStyle {
    return self.style.clone();
  }

  fn get_childs(&self) -> &Vec<RefCellDrawerView> {
    return &self.childs;
  }

  fn get_depth(&self) -> Option<i32> {
    return self.depth;
  }

  fn set_depth(&mut self, depth: Option<i32>) {
    self.depth = depth;
  }

  fn get_parent(&self) -> Option<RefCellDrawerView> {
    return self.parent.clone();
  }
}

impl View {
  pub fn new(style: StyleDeclarations, childs: Vec<View>) -> View {
    let style = Rc::new(RefCell::new(Style::new(style)));
    prepare_node(style.clone(), &childs);

    let childs: Vec<RefCellDrawerView> = childs
      .into_iter()
      .map(|view| Rc::new(RefCell::new(view)) as RefCellDrawerView)
      .collect();

    View {
      parent: None,
      depth: None,
      childs,
      style,
    }
  }
}
