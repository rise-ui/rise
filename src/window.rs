use euclid;
use gleam::gl;
use glutin;
use glutin::GlContext;
use webrender::api::{DeviceUintSize, LayoutSize};

pub struct Window {
  pub window: glutin::GlWindow,
}
impl Window {
  pub fn new(window_builder: glutin::WindowBuilder, events_loop: &glutin::EventsLoop) -> Self {
    let context = glutin::ContextBuilder::new()
      .with_vsync(true)
      // .with_multisampling(1)
      .with_gl(glutin::GlRequest::GlThenGles {
        opengl_version: (3, 2),
        opengles_version: (3, 0)
      });

    let window = glutin::GlWindow::new(window_builder, context, events_loop).unwrap();
    unsafe { window.make_current().ok() };
    Window {
      window: window,
    }
  }
  pub fn gl(&self) -> ::std::rc::Rc<gl::Gl> {
    match gl::GlType::default() {
      gl::GlType::Gl => unsafe { gl::GlFns::load_with(|symbol| self.window.get_proc_address(symbol) as *const _) },
      gl::GlType::Gles => unsafe { gl::GlesFns::load_with(|symbol| self.window.get_proc_address(symbol) as *const _) },
    }
  }
  pub fn swap_buffers(&self) {
    self.window.swap_buffers().ok();
  }
  pub fn hidpi_factor(&self) -> f32 {
    self.window.hidpi_factor()
  }
  pub fn resize(&mut self, width: u32, height: u32) {
    self.window.set_inner_size(width, height);
  }

  pub fn size_px(&self) -> DeviceUintSize {
    let (width, height) = self.window.get_inner_size().unwrap();
    DeviceUintSize::new(width, height)
  }

  pub fn size(&self) -> (f32, f32) {
    let (width, height) = self.window.get_inner_size().unwrap();
    return (width as f32, height as f32);
  }

  pub fn size_dp(&self) -> LayoutSize {
    let (width, height) = self.window.get_inner_size().unwrap();
    let hidpi = self.hidpi_factor();
    LayoutSize::new(width as f32 / hidpi, height as f32 / hidpi)
  }
}
