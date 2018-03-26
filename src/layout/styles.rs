use rsx_primitives::rsx_stylesheet::types::StyleDeclarations;
use rsx_primitives::rsx_stylesheet::types::StyleDeclaration;

use rsx_primitives::rsx_stylesheet::types::ThemeStyle;
use rsx_primitives::rsx_stylesheet::types::FlexStyle;
use rsx_primitives::rsx_stylesheet::types::Color;
use rsx_primitives::rsx_stylesheet::yoga::Node;

use render::RenderBuilder;
use std::cell::RefCell;
use std::rc::Rc;

use webrender::api::*;
use palette;

pub fn token_rgb_to_webrender_color(color: Color) -> ColorF {
  use palette::rgb::{ Rgb, Srgb };
  use palette::Alpha;

  let rgb = Alpha::<Rgb, _>::new_u8(color.red, color.green, color.blue, color.alpha);
  return ColorF::new(rgb.red, rgb.green, rgb.blue, rgb.alpha);
}

pub enum ClipStyleType {
  Background,
  Container
}

// Arguments: point: (left, top), size: (width, height)
pub fn generate_clip_primitive(border_radius: f32, point: (f32, f32), size: (f32, f32), clip_style_type: ClipStyleType) -> PrimitiveInfo<LayerPixel> {
  let point_started = match clip_style_type {
    ClipStyleType::Container => LayoutPoint::new(point.0, point.1),
    ClipStyleType::Background => LayoutPoint::new(0.0, 0.0)
  };

  let bounds = LayoutRect::new(point_started, LayoutSize::new(size.0, size.1));

  let complex_clip = ComplexClipRegion {
    radii: BorderRadius::uniform(border_radius),
    mode: ClipMode::Clip,
    rect: bounds,
  };

  let mut clip = LayoutPrimitiveInfo {
    local_clip: LocalClip::RoundedRect(bounds, complex_clip),
    .. LayoutPrimitiveInfo::new(bounds)
  };

  return clip;
}

#[derive(Debug, Clone)]
pub struct Style {
  declarations: StyleDeclarations,
  pub node: Rc<RefCell<Node>>,
}

impl Style {
  fn create_node_with_layout_styles(style: &StyleDeclarations) -> Node {
    let mut layout: Vec<FlexStyle> = vec![];
    let mut node = Node::new();

    for declaration in style.into_iter() {
      match declaration {
        &StyleDeclaration::Layout(s) => {
          let style = s.clone();
          layout.push(style);
        },
        _ => {}
      }
    }

    node.apply_styles(&layout);
    return node;
  }

  pub fn new(style: StyleDeclarations) -> Style {
    let node = Style::create_node_with_layout_styles(&style);

    Style {
      node: Rc::new(RefCell::new(node)),
      declarations: style,
    }
  }

  pub fn draw(&mut self, builder_context: Rc<RefCell<RenderBuilder>>) {
    let mut theme_styles: Vec<ThemeStyle> = vec![];
    // let mut flex_styles: Vec<FlexStyle> = vec![];

    for declaration in self.declarations.into_iter() {
      match declaration {
        // &StyleDeclaration::Layout(ref s) => flex_styles.push(s.clone()),
        &StyleDeclaration::Theme(ref s) => theme_styles.push(s.clone()),
        _ => {}
      }
    }

    let node = self.node.clone();
    let layout = node.borrow().get_layout();

    // Drawing
    let container_clip = generate_clip_primitive(
      10.0, (layout.left(), layout.top()), (layout.width(), layout.height()),
      ClipStyleType::Container
    );

    builder_context.borrow_mut().builder.push_stacking_context(
      &container_clip,
      ScrollPolicy::Scrollable,
      None,
      TransformStyle::Flat,
      None,
      MixBlendMode::Normal,
      vec![],
    );

    // let details = BorderDetails::Normal(border_details)

    for style in theme_styles.iter() {
      match style {
        &ThemeStyle::BackgroundColor(color) => {
          let prepared_color = token_rgb_to_webrender_color(color.clone());
          let background_clip = generate_clip_primitive(
            10.0, (layout.left(), layout.top()), (layout.width(), layout.height()),
            ClipStyleType::Background
          );

          builder_context.borrow_mut().builder.push_rect(&background_clip, prepared_color);
        },
        _ => {}
      }
    }

    // builder_context.borrow_mut().builder.pop_stacking_context();
  }
}