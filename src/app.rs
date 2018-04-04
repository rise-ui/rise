use glutin::{self, Event, EventsLoop, WindowBuilder};
use std::cell::RefCell;
use std::rc::Rc;

use std::time::{Duration, Instant};

// Core contexts
use layout::layout::Layout;
use render::WebRenderContext;
use ui::Ui;
use window::Window;

pub enum WindowPosition {
  MiddleRight,
  MiddleLeft,
  Center,
}

pub struct WindowOptions {
  pub position: WindowPosition,
  pub window_size: (u32, u32),
  pub title: String,
}

impl WindowOptions {
  pub fn get_window_position(&self, monitor: glutin::MonitorId) -> (i32, i32) {
    let mut position: (i32, i32) = (0, 0);

    let hidpi = monitor.get_hidpi_factor();
    let window_size = &self.window_size;

    let monitor_size = monitor.get_dimensions();
    let monitor_size = (monitor_size.0 as f32 / hidpi, monitor_size.1 as f32 / hidpi);

    match &self.position {
      &WindowPosition::MiddleLeft => {
        position = (0, ((monitor_size.1 - window_size.1 as f32) / 2.0) as i32)
      }
      &WindowPosition::MiddleRight => {
        position = (
          (monitor_size.1 - window_size.1 as f32) as i32,
          ((monitor_size.1 - window_size.1 as f32) / 2.0) as i32,
        )
      }
      &WindowPosition::Center => {
        position = (
          ((monitor_size.0 - window_size.0 as f32) / 2.0) as i32,
          ((monitor_size.1 - window_size.1 as f32) / 2.0) as i32,
        )
      }
    }

    return position;
  }
}

impl Default for WindowOptions {
  fn default() -> WindowOptions {
    WindowOptions {
      position: WindowPosition::Center,
      window_size: (600, 400),
      title: String::from(""),
    }
  }
}

pub struct App {
  event_loop: Rc<RefCell<EventsLoop>>,
  window_initialized: bool,
  frame_time: Instant,
  ui: Ui,
}

impl App {
  pub fn new(options: WindowOptions, layout: Layout) -> App {
    let window_builder = WindowBuilder::new()
      .with_dimensions(options.window_size.0, options.window_size.1)
      .with_title(&*options.title)
      .with_decorations(false)
      .with_transparency(true)
      .with_multitouch();

    let event_loop = EventsLoop::new();
    let mut window = Window::new(window_builder, &event_loop);

    let position = options.get_window_position(event_loop.get_primary_monitor());
    window.window.set_position(position.0, position.1);

    let webrender_context = WebRenderContext::new(&mut window, &event_loop);
    let event_loop = Rc::new(RefCell::new(event_loop));
    let ui = Ui::new(event_loop.clone(), webrender_context, window, layout);

    App {
      window_initialized: false,
      frame_time: Instant::now(),
      event_loop,
      ui,
    }
  }

  fn tick_frame_time(&mut self, log: bool) {
    use std::io::{self, Write};

    let elapsed_time = self.frame_time.elapsed();
    let elapsed_ms =
      (elapsed_time.as_secs() * 1_000) + (elapsed_time.subsec_nanos() / 1_000_000) as u64;

    if log {
      let text = format!("\rframe time: {:?}ms", elapsed_ms);
      let stdout = io::stdout();
      let mut handle = stdout.lock();

      if let Ok(_) = handle.write(text.as_bytes()) {}
    }

    self.frame_time = Instant::now();
  }

  pub fn run(mut self) {
    let event_loop = Rc::clone(&self.event_loop);
    let mut event_loop = event_loop.borrow_mut();

    self.window_initialized = true;

    loop {
      if !self.ui.needs_redraw() {
        event_loop.run_forever(|event| {
          self.handle_window_event(event);
          glutin::ControlFlow::Break
        });
      }

      event_loop.poll_events(|event| {
        self.handle_window_event(event);
      });

      if self.ui.should_close() {
        self.ui.close_app();
        return;
      }

      self.tick_frame_time(false);
      self.ui.update();
    }
  }

  fn handle_window_event(&mut self, event: Event) {}
}
