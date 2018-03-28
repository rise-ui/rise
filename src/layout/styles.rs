use rsx_shared::traits::TComputedStyles;
use rsx_shared::types::KnownElementName;
use rsx_stylesheet::types::{Color, ComputedStyles, FlexStyle, StyleDeclaration, StyleDeclarations,
                            StyleUnit};
use rsx_stylesheet::yoga::Node;

use render::RenderBuilder;
use std::cell::RefCell;
use std::rc::Rc;

use webrender::api::*;

pub fn token_rgb_to_webrender_color(color: Color) -> ColorF {
  use palette::Alpha;
  use palette::rgb::{Rgb, Srgb};

  let rgb = Alpha::<Rgb, _>::new_u8(color.red, color.green, color.blue, color.alpha);
  return ColorF::new(rgb.red, rgb.green, rgb.blue, rgb.alpha);
}

pub enum ClipStyleType {
  Background,
  Container,
}

// Arguments: point: (left, top), size: (width, height)
fn generate_clip_primitive(
  point: &(f32, f32),
  size: &(f32, f32),
  border_radius: BorderRadius,
  clip_style_type: ClipStyleType,
) -> PrimitiveInfo<LayerPixel> {
  let point_started = match clip_style_type {
    ClipStyleType::Container => LayoutPoint::new(point.0, point.1),
    ClipStyleType::Background => LayoutPoint::new(0.0, 0.0),
  };

  let bounds = LayoutRect::new(point_started, LayoutSize::new(size.0, size.1));

  let complex_clip = ComplexClipRegion {
    radii: border_radius,
    mode: ClipMode::Clip,
    rect: bounds,
  };

  let clip = LayoutPrimitiveInfo {
    local_clip: LocalClip::RoundedRect(bounds, complex_clip),
    ..LayoutPrimitiveInfo::new(bounds)
  };

  return clip;
}

// TODO: adding support for percent value
fn generate_corner_radius(radius: StyleUnit) -> LayerSize {
  match radius {
    StyleUnit::Point(radius) => LayerSize::new(radius.into_inner(), radius.into_inner()),
    _ => LayerSize::new(0.0, 0.0),
  }
}

fn generate_borders(
  styles: &ComputedStyles,
  border_radius: BorderRadius,
) -> (BorderWidths, BorderDetails) {
  use rsx_stylesheet::types::BorderStyle as RsxBorderStyle;
  enum Position {
    Top,
    Right,
    Bottom,
    Left,
  };

  fn get_border_side(styles: &ComputedStyles, position: Position) -> BorderSide {
    let border_style = match position {
      Position::Bottom => styles.border_bottom_style,
      Position::Right => styles.border_right_style,
      Position::Left => styles.border_left_style,
      Position::Top => styles.border_top_style,
    };

    let style = match border_style {
      RsxBorderStyle::None => BorderStyle::None,
      RsxBorderStyle::Solid => BorderStyle::Solid,
      RsxBorderStyle::Double => BorderStyle::Double,
      RsxBorderStyle::Dotted => BorderStyle::Dotted,
      RsxBorderStyle::Dashed => BorderStyle::Dashed,
      RsxBorderStyle::Hidden => BorderStyle::Hidden,
      RsxBorderStyle::Groove => BorderStyle::Groove,
      RsxBorderStyle::Ridge => BorderStyle::Ridge,
      RsxBorderStyle::Inset => BorderStyle::Inset,
      RsxBorderStyle::Outset => BorderStyle::Outset,
    };

    let color = match position {
      Position::Bottom => token_rgb_to_webrender_color(styles.border_bottom_color),
      Position::Right => token_rgb_to_webrender_color(styles.border_right_color),
      Position::Left => token_rgb_to_webrender_color(styles.border_left_color),
      Position::Top => token_rgb_to_webrender_color(styles.border_top_color),
    };

    return BorderSide { color, style };
  }

  let border_widths = BorderWidths {
    bottom: styles.border_bottom_width as f32,
    right: styles.border_right_width as f32,
    left: styles.border_left_width as f32,
    top: styles.border_top_width as f32,
  };

  let border_details = BorderDetails::Normal(NormalBorder {
    bottom: get_border_side(&styles, Position::Bottom),
    right: get_border_side(&styles, Position::Right),
    left: get_border_side(&styles, Position::Left),
    top: get_border_side(&styles, Position::Top),
    radius: border_radius,
  });

  return (border_widths, border_details);
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
        }
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
    let node = self.node.clone();
    let layout = node.borrow().get_layout();

    // Compute theme styles
    let mut styles = ComputedStyles::make_initial_computed_styles(KnownElementName::Div);
    styles.apply_styles(&self.declarations);

    // Declare sizes as core types
    let size = (layout.width(), layout.height());
    let point = (layout.left(), layout.top());

    // Border Radius prepares
    let mut border_radius = BorderRadius::zero();
    border_radius.bottom_right = generate_corner_radius(styles.border_bottom_right_radius);
    border_radius.bottom_left = generate_corner_radius(styles.border_bottom_left_radius);
    border_radius.top_right = generate_corner_radius(styles.border_top_right_radius);
    border_radius.top_left = generate_corner_radius(styles.border_top_left_radius);

    // Drawing Context Container
    let container_clip = generate_clip_primitive(
      &point,
      &size,
      border_radius.clone(),
      ClipStyleType::Container,
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

    // Drawing Background Rect
    let background_clip = generate_clip_primitive(
      &point,
      &size,
      border_radius.clone(),
      ClipStyleType::Background,
    );

    let prepared_color = token_rgb_to_webrender_color(styles.background_color);
    builder_context
      .borrow_mut()
      .builder
      .push_rect(&background_clip, prepared_color);

    // Drawing borders
    let border = generate_borders(&styles, border_radius.clone());

    builder_context
      .borrow_mut()
      .builder
      .push_border(&background_clip, border.0, border.1);
  }
}
