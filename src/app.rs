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
use render::{ WebRenderContext, RenderBuilder };
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
      .with_dimensions(1000, 1000);

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

//      experemental_draw_context(
//        builder_context.clone(),
//        webrender_context.clone(),
//        window.clone()
//      );

      // Render blocks
      self.layout.borrow_mut().calculate(window.borrow().size());

      self.layout.borrow_mut().render(builder_context.clone());
      builder_context.borrow_mut().builder.pop_stacking_context();

      println!("\nNodes\n");
      trace_nodes(&self.layout.borrow_mut().root, 0);
      builder_context.borrow_mut().builder.print_display_list();

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

// Experiments of drawing on raw builder state
fn experemental_draw_context(builder_context: Rc<RefCell<RenderBuilder>>, webrender_context: Rc<RefCell<WebRenderContext>>, window: Rc<RefCell<Window>>) {
  fn draw_view_context(builder_context: Rc<RefCell<RenderBuilder>>, color: ColorF, position: (f32, f32), size: (f32, f32)) {
    let bounds = LayoutRect::new(
      LayoutPoint::new(position.0, position.1),
      LayoutSize::new(size.0, size.1),
    );

    let mut complex_clip = ComplexClipRegion {
      radii: BorderRadius::uniform(3.0),
      mode: ClipMode::Clip,
      rect: bounds,
    };

    let mut container = LayoutPrimitiveInfo {
      local_clip: LocalClip::RoundedRect(bounds, complex_clip),
      .. LayoutPrimitiveInfo::new(bounds)
    };

    builder_context.borrow_mut().builder.push_stacking_context(
      &container,
      ScrollPolicy::Scrollable,
      None,
      TransformStyle::Flat,
      None,
      MixBlendMode::Normal,
      vec![],
    );

    builder_context.borrow_mut().builder.push_rect(&container, color);
  }

  //  Draw main

  // Root(childs)
  draw_view_context(
    builder_context.clone(),
    ColorF::new(1.0, 1.0, 1.0, 1.0),
    (10.0, 10.0), (600.0, 600.0)
  );

  // Child(by: root)(id: 0)
  draw_view_context(
    builder_context.clone(),
    ColorF::new(0.13, 0.59, 0.95, 1.0),
    (10.0, 10.0), (100.0, 100.0)
  );

    // Child(by: Child(by: root)(id: 0))(id: 0)
    draw_view_context(
      builder_context.clone(),
      ColorF::new(1.0, 0.34, 0.13, 1.0),
      (10.0, 10.0), (30.0, 30.0)
    );

    // Back current context to : Child(by: root)(id: 0) from current context
    builder_context.borrow_mut().builder.pop_stacking_context();

    // Child(by: Child(by: root)(id: 0))(id: 1)
    draw_view_context(
      builder_context.clone(),
      ColorF::new(0.0, 0.0, 0.0, 1.0),
      (35.0, 10.0), (30.0, 30.0)
    );

    // Back current context to : Child(by: root)(id: 0) from current context
    builder_context.borrow_mut().builder.pop_stacking_context();

  // Back current context to : Roots(Child)from current context
  builder_context.borrow_mut().builder.pop_stacking_context();

  // Roots(Child)
  draw_view_context(
    builder_context.clone(),
    ColorF::new(0.13, 0.12, 0.39, 1.0),
    (100.0, 10.0), (100.0, 100.0)
  );

  // Apply builder list to view on screen
  println!("\n Trace Webrender Nodes\n");
  builder_context.borrow_mut().builder.print_display_list();

  webrender_context.borrow_mut().set_display_list(
    builder_context.borrow().builder.clone(),
    builder_context.borrow().resources.clone(),
    window.borrow().size_dp()
  );

  webrender_context.borrow_mut().update(window.borrow().size_px());
}