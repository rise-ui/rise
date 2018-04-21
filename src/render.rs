pub use rise_stylesheet::common::RenderBuilder;
use std::sync::atomic::{self, AtomicBool};
use std::sync::Arc;

use gleam::gl;
use glutin;
use webrender;
use webrender::api::*;

use euclid::TypedPoint2D;
use geometry::{Point, Rect, Size};
use resources;
use window::Window;

pub struct WebRenderContext {
  pub renderer: webrender::Renderer,
  pub render_api: RenderApi,
  pub epoch: Epoch,
  pub pipeline_id: PipelineId,
  pub document_id: DocumentId,
  pub device_pixel_ratio: f32,
  pub root_background_color: ColorF,
  pub frame_ready: Arc<AtomicBool>,
}

impl WebRenderContext {
  pub fn new(window: &mut Window, events_loop: &glutin::EventsLoop) -> Self {
    let gl = window.gl();
    println!("OpenGL version {}", gl.get_string(gl::VERSION));
    println!("HiDPI factor {}", window.hidpi_factor());

    let opts = webrender::RendererOptions {
      clear_color: Some(ColorF::new(0.0, 0.0, 0.0, 0.01)),
      device_pixel_ratio: window.hidpi_factor(),
      resource_override_path: None,
      enable_subpixel_aa: true,
      precache_shaders: false,
      ..webrender::RendererOptions::default()
    };

    let frame_ready = Arc::new(AtomicBool::new(false));
    let notifier = Box::new(Notifier::new(
      events_loop.create_proxy(),
      Arc::clone(&frame_ready),
    ));

    let (mut renderer, sender) = webrender::Renderer::new(gl, notifier, opts).unwrap();
    let api = sender.create_api();
    resources::init_resources(sender);
    let document_id = api.add_document(window.size_px(), 0);

    renderer.set_external_image_handler(Box::new(QuickExternalImageHandler));

    let epoch = Epoch(0);
    let root_background_color = ColorF::new(0.0, 0.0, 0.0, 0.0);
    let pipeline_id = PipelineId(0, 0);

    let mut txn = Transaction::new();
    txn.set_root_pipeline(pipeline_id);
    api.send_transaction(document_id, txn);

    WebRenderContext {
      renderer: renderer,
      render_api: api,
      epoch: epoch,
      pipeline_id: pipeline_id,
      document_id: document_id,
      device_pixel_ratio: window.hidpi_factor(),
      root_background_color: root_background_color,
      frame_ready: frame_ready,
    }
  }

  pub fn deinit(self) {
    self.renderer.deinit();
  }

  pub fn render_builder(&mut self, window_size: LayoutSize) -> RenderBuilder {
    let builder = DisplayListBuilder::new(self.pipeline_id, window_size);
    RenderBuilder {
      builder: builder,
      resources: ResourceUpdates::new(),
    }
  }

  pub fn set_display_list(
    &mut self,
    builder: DisplayListBuilder,
    resources: ResourceUpdates,
    window_size: LayoutSize,
  ) {
    let mut txn = Transaction::new();
    txn.set_display_list(self.epoch, None, window_size, builder.finalize(), true);

    txn.update_resources(resources);
    txn.generate_frame();

    self.render_api.send_transaction(self.document_id, txn);
  }

  pub fn generate_frame(&mut self) {
    let mut txn = Transaction::new();
    txn.generate_frame();
    self.render_api.send_transaction(self.document_id, txn);
  }

  pub fn frame_ready(&mut self) -> bool {
    self.frame_ready.load(atomic::Ordering::Acquire)
  }

  // if there is a frame ready, update current frame and render it, otherwise, does nothing
  pub fn update(&mut self, window_size: DeviceUintSize) {
    self.frame_ready.store(false, atomic::Ordering::Release);
    self.renderer.update();
    self.renderer.render(window_size).unwrap();
    let _ = self.renderer.flush_pipeline_info();
  }

  pub fn toggle_flags(&mut self, toggle_flags: webrender::DebugFlags) {
    let mut flags = self.renderer.get_debug_flags();
    flags.toggle(toggle_flags);
    self.renderer.set_debug_flags(flags);
  }

  pub fn window_resized(&mut self, size: DeviceUintSize) {
    let window_rect = DeviceUintRect::new(TypedPoint2D::zero(), size);
    self.render_api.set_window_parameters(
      self.document_id,
      size,
      window_rect,
      self.device_pixel_ratio,
    );
  }
}

struct Notifier {
  events_proxy: glutin::EventsLoopProxy,
  frame_ready: Arc<AtomicBool>,
}
impl Notifier {
  fn new(events_proxy: glutin::EventsLoopProxy, frame_ready: Arc<AtomicBool>) -> Self {
    Notifier {
      events_proxy: events_proxy,
      frame_ready: frame_ready,
    }
  }
}

impl RenderNotifier for Notifier {
  fn wake_up(&self) {
    #[cfg(not(target_os = "android"))]
    self.events_proxy.wakeup().ok();
    self.frame_ready.store(true, atomic::Ordering::Release);
  }

  fn new_document_ready(&self, _: DocumentId, _: bool, _: bool) {
    self.wake_up();
  }

  fn clone(&self) -> Box<RenderNotifier + 'static> {
    Box::new(Notifier::new(
      self.events_proxy.clone(),
      self.frame_ready.clone(),
    ))
  }
}

struct QuickExternalImageHandler;

impl webrender::ExternalImageHandler for QuickExternalImageHandler {
  // Do not perform any actual locking since rendering happens on the main thread
  fn lock(&mut self, key: ExternalImageId, _channel_index: u8) -> webrender::ExternalImage {
    let descriptor = resources::resources().image_loader.texture_descriptors[&key.0];

    webrender::ExternalImage {
      uv: TexelRect {
        uv0: TypedPoint2D::zero(),
        uv1: TypedPoint2D::<f32, DevicePixel>::new(
          descriptor.width as f32,
          descriptor.height as f32,
        ),
      },
      source: webrender::ExternalImageSource::NativeTexture(key.0 as _),
    }
  }

  fn unlock(&mut self, _key: ExternalImageId, _channel_index: u8) {}
}
