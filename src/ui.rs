// Layout
use glutin::EventsLoop;
use layout::Layout;
use solver::prelude::Shortcuts;

use std::cell::RefCell;
use std::rc::Rc;

// Core contexts
use render::{RenderBuilder, WebRenderContext};
use window::Window;

pub struct Ui {
  pub render: Rc<RefCell<WebRenderContext>>,
  pub shortcuts: Rc<RefCell<Shortcuts>>,
  event_loop: Rc<RefCell<EventsLoop>>,
  pub window: Rc<RefCell<Window>>,
  pub layout: Rc<RefCell<Layout>>,
  needs_redraw: bool,
  should_close: bool,
}

impl Ui {
  pub fn new(event_loop: Rc<RefCell<EventsLoop>>, render: WebRenderContext, window: Window, layout: Layout) -> Ui {
    Ui {
      shortcuts: Rc::new(RefCell::new(Shortcuts::new())),
      render: Rc::new(RefCell::new(render)),
      window: Rc::new(RefCell::new(window)),
      layout: Rc::new(RefCell::new(layout)),
      should_close: false,
      needs_redraw: true,
      event_loop,
    }
  }

  pub fn redraw(&mut self) {
    self.needs_redraw = true;
  }

  pub fn needs_redraw(&self) -> bool {
    self.needs_redraw
  }

  pub fn should_close(&self) -> bool {
    self.should_close
  }

  pub fn close_app(self) {
    let render = Rc::clone(&self.render);
    let render = render.borrow();
    // @TODO: Fix close app
    // render.deinit();
  }

  pub fn update(&mut self) {
    let render = self.render.clone();
    let window = self.window.clone();

    let builder_context = render.borrow_mut().render_builder(self.window.borrow().size_dp());

    let builder_context = Rc::new(RefCell::new(builder_context));

    // Render blocks
    self.layout.borrow_mut().calculate(window.borrow().size());
    self.layout.borrow_mut().render(builder_context.clone());
    builder_context.borrow_mut().builder.pop_stacking_context();

    render.borrow_mut().set_display_list(
      builder_context.borrow().builder.clone(),
      builder_context.borrow().resources.clone(),
      window.borrow().size_dp(),
    );

    render.borrow_mut().update(window.borrow().size_px());

    window.borrow_mut().swap_buffers();
  }
}
