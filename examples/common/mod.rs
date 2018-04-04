use ordered_float::OrderedFloat;
use rsx_stylesheet::types::Stylesheet;
use rsx_stylesheet::types::*;
use rsx_stylesheet::*;

// HardFix, current `style!` macros are not support global `border-radius` property, bot support radius by corner
pub fn add_border_radius_to_all(
  mut style: StyleDeclarations,
  border_radius: f32,
) -> StyleDeclarations {
  style.push(StyleDeclaration::Theme(ThemeStyle::BorderTopLeftRadius(
    StyleUnit::Point(OrderedFloat(border_radius)),
  )));
  style.push(StyleDeclaration::Theme(ThemeStyle::BorderTopRightRadius(
    StyleUnit::Point(OrderedFloat(border_radius)),
  )));
  style.push(StyleDeclaration::Theme(
    ThemeStyle::BorderBottomRightRadius(StyleUnit::Point(OrderedFloat(border_radius))),
  ));
  style.push(StyleDeclaration::Theme(ThemeStyle::BorderBottomLeftRadius(
    StyleUnit::Point(OrderedFloat(border_radius)),
  )));

  return style;
}

// HardFix, current style! macros are not support border-* properties and throw on compile
pub fn add_border_to_all(
  mut style: StyleDeclarations,
  width: f32,
  color: Color,
) -> StyleDeclarations {
  style.push(StyleDeclaration::Layout(FlexStyle::BorderTop(
    OrderedFloat(width),
  )));

  style.push(StyleDeclaration::Theme(ThemeStyle::BorderTopColor(color)));

  style.push(StyleDeclaration::Theme(ThemeStyle::BorderTopStyle(
    BorderStyle::Solid,
  )));

  return style;
}
