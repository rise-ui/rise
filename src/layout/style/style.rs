use rsx_shared::traits::TComputedStyles;
use rsx_shared::types::KnownElementName;
use rsx_stylesheet::types::{ComputedStyles, StyleDeclarations};
use rsx_stylesheet::yoga::Node;

use render::RenderBuilder;
use std::cell::RefCell;
use std::rc::Rc;

use webrender::api::*;

use layout::style::prelude::{create_node_with_layout_styles, generate_borders,
                             generate_clip_primitive, generate_corner_radius,
                             token_rgb_to_webrender_color, ClipStyleType};

use layout::style::prelude::DrawerStyle;

pub type RefCellDrawerStyle = Rc<RefCell<DrawerStyle>>;

#[derive(Debug, Clone)]
pub struct Style {
  declarations: StyleDeclarations,
  pub node: Rc<RefCell<Node>>,
}

impl Style {
  pub fn new(style: StyleDeclarations) -> Style {
    let node = create_node_with_layout_styles(&style);

    Style {
      node: Rc::new(RefCell::new(node)),
      declarations: style,
    }
  }
}

impl DrawerStyle for Style {
  fn get_styles(&self) -> StyleDeclarations {
    let styles = self.declarations.clone();
    return styles;
  }

  fn get_node(&self) -> Rc<RefCell<Node>> {
    let node = self.node.clone();
    return node;
  }

  fn draw(&mut self, builder_context: Rc<RefCell<RenderBuilder>>) {
    let node = self.get_node();
    let layout = node.borrow().get_layout();

    // Compute theme styles
    let mut styles = ComputedStyles::make_initial_computed_styles(KnownElementName::Div);
    styles.apply_styles(&self.get_styles());

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

    // builder_context.borrow_mut().builder.pop_stacking_context();
  }
}
