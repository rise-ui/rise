use super::webrender::api::{DeviceIntSize, LayoutSize};
use glutin::GlContext;
use gleam::gl;
use glutin;

pub struct Window {
    pub window: glutin::GlWindow,
}

impl Window {
    pub fn new(window_builder: glutin::WindowBuilder, events_loop: &glutin::EventsLoop) -> Self {
        let context = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::GlThenGles {
                opengl_version: (3, 2),
                opengles_version: (3, 0),
            });

        let window = glutin::GlWindow::new(window_builder, context, events_loop).unwrap();
        unsafe { window.make_current().ok() };
        
        Window {
            window: window,
        }
    }

    pub fn gl(&self) -> ::std::rc::Rc<gl::Gl> {
        match gl::GlType::default() {
            gl::GlType::Gl => unsafe {
                gl::GlFns::load_with(|symbol| self.window.get_proc_address(symbol) as *const _)
            },
            
            gl::GlType::Gles => unsafe {
                gl::GlesFns::load_with(|symbol| self.window.get_proc_address(symbol) as *const _)
            },
        }
    }

    pub fn swap_buffers(&self) {
        self.window.swap_buffers().ok();
    }

    pub fn hidpi_factor(&self) -> f32 {
        self.window.get_hidpi_factor() as f32
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.window.set_inner_size((width, height).into());
    }

    pub fn size_px(&self) -> DeviceIntSize {
        let size = self.window.get_inner_size().unwrap();
        let hidpi = self.hidpi_factor();

        let width = size.width as f32 * hidpi;
        let height = size.height as f32 * hidpi;
        DeviceIntSize::new(width as i32, height as i32)
    }

    pub fn size(&self) -> (f32, f32) {
        let size = self.window.get_inner_size().unwrap();
        return (size.width as f32, size.height as f32);
    }

    pub fn size_dp(&self) -> LayoutSize {
        let size = self.window.get_inner_size().unwrap();

        LayoutSize::new(size.width as f32, size.height as f32)
    }
}
