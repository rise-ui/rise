use render::RenderBuilder;
use rise_stylesheet::styles::style::{Style, StyleExt};
use rise_stylesheet::yoga::context::{get_context, get_context_mut, ContextExt, NodeContextExt};
use rise_stylesheet::yoga::yoga_sys;
use rise_stylesheet::yoga::Node;
use std::any::Any;
use std::boxed::Box;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct NodeContext {
  pub style: Box<StyleExt>,
  pub data: Box<Any>,
}

impl ContextExt for NodeContext {
}

pub trait StyleContextExt {
  fn draw(&self, render: Rc<RefCell<RenderBuilder>>) -> Option<()>;
  fn get_mut_styles(&self) -> Option<&mut Box<StyleExt>>;
  fn render(&self, render: Rc<RefCell<RenderBuilder>>);
  fn get_styles(&self) -> Option<&Box<StyleExt>>;
}

impl StyleContextExt for Node {
  fn get_styles(&self) -> Option<&Box<StyleExt>> {
    let context_any = get_context(&self.inner_node)?;
    let context = context_any.downcast_ref::<NodeContext>()?;
    Some(&context.style)
  }

  fn get_mut_styles(&self) -> Option<&mut Box<StyleExt>> {
    let context_any = get_context_mut(&self.inner_node)?;
    let context = context_any.downcast_mut::<NodeContext>()?;
    Some(&mut context.style)
  }

  fn draw(&self, render: Rc<RefCell<RenderBuilder>>) -> Option<()> {
    let styles = self.get_mut_styles()?;
    styles.draw(&self, render);

    Some(())
  }

  fn render(&self, render: Rc<RefCell<RenderBuilder>>) {
    self.draw(render.clone());

    if let Some(childrens) = self.childrens() {
      for children in childrens.iter() {
        children.render(render.clone());

        render.borrow_mut().builder.pop_stacking_context();
        render.borrow_mut().builder.pop_clip_id();
      }
    }
  }
}

impl Deref for NodeContext {
  type Target = Box<Any>;
  fn deref(&self) -> &Box<Any> {
    &self.data
  }
}

impl NodeContextExt<NodeContext> for Node {
  fn new_with_context(context: NodeContext) -> Node {
    let mut node = Node::new();
    node.set_context(Some(context));
    node
  }

  fn set_context(&mut self, value: Option<NodeContext>) {
    use std::os::raw::c_void;

    let prev_raw = unsafe { yoga_sys::YGNodeGetContext(self.inner_node) };
    NodeContext::drop_raw(prev_raw);

    let raw: *mut c_void = value.map_or_else(|| ::std::ptr::null_mut(), |context| NodeContext::into_raw(Box::new(context)));
    unsafe { yoga_sys::YGNodeSetContext(self.inner_node, raw) }
  }

  fn get_own_context(&self) -> Option<&Box<Any>> {
    let context_any = get_context(&self.inner_node)?;
    let context = context_any.downcast_ref::<NodeContext>()?;
    Some(&context.data)
  }

  fn get_own_context_mut(&self) -> Option<&mut Box<Any>> {
    let context_any = get_context_mut(&self.inner_node)?;
    let context = context_any.downcast_mut::<NodeContext>()?;
    Some(&mut context.data)
  }

  fn drop_context(&mut self) {
    // let prev_raw = unsafe { yoga_sys::YGNodeGetContext(self.inner_node) };
    // NodeContext::drop_raw(prev_raw);
  }
}
