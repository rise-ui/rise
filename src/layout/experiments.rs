use std::cell::RefCell;
use std::rc::Rc;
use webrender::api::*;

use render::{RenderBuilder, WebRenderContext};
use window::Window;

// Experiments of drawing on raw builder state
pub fn draw_raw_experiment(
  builder_context: Rc<RefCell<RenderBuilder>>,
  webrender_context: Rc<RefCell<WebRenderContext>>,
  window: Rc<RefCell<Window>>,
) {
  fn draw_view_context(
    builder_context: Rc<RefCell<RenderBuilder>>,
    color: ColorF,
    position: (f32, f32),
    size: (f32, f32),
    is_gradient: bool,
  ) {
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
      ..LayoutPrimitiveInfo::new(bounds)
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

    let stops = vec![
      GradientStop {
        offset: 0.0,
        color: ColorF::new(0.84, 0.2, 0.41, 1.0),
      },
      GradientStop {
        offset: 0.5,
        color: ColorF::new(0.8, 0.68, 0.43, 1.0),
      },
    ];

    let gradient = builder_context.borrow_mut().builder.create_gradient(
      LayoutPoint::new(position.0, position.1),
      LayoutPoint::new(position.0 + size.0, position.1 + size.1),
      stops,
      ExtendMode::Clamp,
    );

    if is_gradient {
      builder_context.borrow_mut().builder.push_gradient(
        &container,
        gradient,
        LayoutSize::new(size.0, size.1),
        LayoutSize::new(0.0, 0.0),
      );
    } else {
      builder_context
        .borrow_mut()
        .builder
        .push_rect(&container, color);
    }
  }

  //  Draw main

  // Root(childs)
  draw_view_context(
    builder_context.clone(),
    ColorF::new(1.0, 1.0, 1.0, 1.0),
    (10.0, 10.0),
    (600.0, 600.0),
    false,
  );

  // Child(by: root)(id: 0)
  draw_view_context(
    builder_context.clone(),
    ColorF::new(0.13, 0.59, 0.95, 1.0),
    (10.0, 10.0),
    (100.0, 100.0),
    false,
  );

  // Child(by: Child(by: root)(id: 0))(id: 0)
  draw_view_context(
    builder_context.clone(),
    ColorF::new(1.0, 0.34, 0.13, 1.0),
    (10.0, 10.0),
    (30.0, 30.0),
    false,
  );

  // Back current context to : Child(by: root)(id: 0) from current context
  builder_context.borrow_mut().builder.pop_stacking_context();

  // Child(by: Child(by: root)(id: 0))(id: 1)
  draw_view_context(
    builder_context.clone(),
    ColorF::new(0.0, 0.0, 0.0, 1.0),
    (35.0, 10.0),
    (30.0, 30.0),
    false,
  );

  // Back current context to : Child(by: root)(id: 0) from current context
  builder_context.borrow_mut().builder.pop_stacking_context();

  // Back current context to : Roots(Child)from current context
  builder_context.borrow_mut().builder.pop_stacking_context();

  // Roots(Child)
  draw_view_context(
    builder_context.clone(),
    ColorF::new(0.13, 0.12, 0.39, 1.0),
    (100.0, 10.0),
    (100.0, 100.0),
    true,
  );

  // Apply builder list to view on screen
  println!("\n Trace Webrender Nodes\n");
  builder_context.borrow_mut().builder.print_display_list();

  webrender_context.borrow_mut().set_display_list(
    builder_context.borrow().builder.clone(),
    builder_context.borrow().resources.clone(),
    window.borrow().size_dp(),
  );

  webrender_context
    .borrow_mut()
    .update(window.borrow().size_px());
}
