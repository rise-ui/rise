use glutin::{ self, GlContext, EventsLoop, WindowBuilder };
use webrender::api::*;
use gleam::gl;

use std::cell::RefCell;
use std::rc::Rc;

// Layout
use layout::layout::{ Layout, trace_nodes };
use layout::styles::Style;
use layout::view::View;

// Core contexts
use render::WebRenderContext;
use window::Window;

pub struct App {
  webrender_context: Rc<RefCell<WebRenderContext>>,
  event_loop: Rc<RefCell<EventsLoop>>,
  window: Rc<RefCell<Window>>,
  layout: Rc<RefCell<Layout>>
}

impl App {
  pub fn new(layout: Layout) -> App {
    let event_loop = EventsLoop::new();
    let window_builder = WindowBuilder::new()
      .with_title("My Example App")
      .with_multitouch()
      .with_decorations(false)
      .with_transparency(true)
      .with_dimensions(700, 700);

    let mut window = Window::new(window_builder, &event_loop);
    let webrender_context = WebRenderContext::new(&mut window, &event_loop);

    App {
      webrender_context: Rc::new(RefCell::new(webrender_context)),
      event_loop: Rc::new(RefCell::new(event_loop)),
      window: Rc::new(RefCell::new(window)),
      layout: Rc::new(RefCell::new(layout)),
    }
  }

  pub fn run(&self) {
    let webrender_context = self.webrender_context.clone();
    let window = self.window.clone();
    let layout = self.layout.clone();

    self.event_loop.borrow_mut().run_forever(move |global_event| {
      let mut transaction = Transaction::new();

      match global_event {
        glutin::Event::WindowEvent { event, .. } => match event {
          // TODO: bug - glutin not return close event
          glutin::WindowEvent::Closed => {
            return glutin::ControlFlow::Break;
          },

          glutin::WindowEvent::Resized(w, h) => {
            let size = DeviceUintSize::new(w, h);
            webrender_context.borrow_mut().window_resized(size);
          },

          _ => (),
        },

        _ => ()
      }

      let builder_context = webrender_context.borrow_mut().render_builder(window.borrow().size_dp());
      let builder_context = Rc::new(RefCell::new(builder_context));

      // Render blocks
      self.layout.borrow_mut().calculate(window.borrow().size());
      // trace_nodes(&self.layout.borrow().root, 0);
      self.layout.borrow_mut().render(builder_context.clone());

      webrender_context.borrow_mut().set_display_list(
        builder_context.borrow().builder.clone(),
        builder_context.borrow().resources.clone(),
        window.borrow().size_dp()
      );
      webrender_context.borrow_mut().update(window.borrow().size_px());
      window.borrow_mut().swap_buffers();

      glutin::ControlFlow::Continue
    });

//    @TODO Fix `move out of borrowed content`
//    webrender_context.borrow_mut().deinit();
  }
}