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

fn token_rgb_to_webrender_color(color: Color) -> ColorF {
  use palette::rgb::{ Rgb, Srgb };
  use palette::Alpha;

  let red = (color.red / 255) as f32;
  let green = (color.green / 255) as f32;
  let blue = (color.blue / 255) as f32;
  let alpha = (color.alpha / 255) as f32;

//  let rgb = Alpha::<Rgb<Srgb, f32>, f32>::new_u8(color.red, color.green, color.blue, color.alpha);
  return ColorF::new(red, green, blue, alpha);
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
    let mut builder = builder_context.borrow_mut().builder.clone();
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
    let bounds = LayoutRect::new(
      LayoutPoint::new(layout.left(), layout.top()),
      LayoutSize::new(layout.width(), layout.height()),
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

    let filters = vec![
      // FilterOp::Opacity(PropertyBinding::Binding(self.opacity_key), self.opacity),
    ];

    builder.push_stacking_context(
      &container,
      ScrollPolicy::Scrollable,
      None,
      TransformStyle::Flat,
      None,
      MixBlendMode::Normal,
      filters,
    );

    // let details = BorderDetails::Normal(border_details)
    for style in theme_styles.iter() {
      match style {
        &ThemeStyle::BackgroundColor(color) => {
          let prepared_color = token_rgb_to_webrender_color(color.clone());
          builder.push_rect(&container, prepared_color);
        },
        _ => {}
      }
    }

    builder.push_rect(&container, ColorF::new(1.0, 1.0, 1.0, 1.0));
    builder.pop_stacking_context();
  }
}