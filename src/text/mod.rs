use resources;
use text_layout::{self, Align, Wrap};

#[derive(Debug, Clone)]
pub struct TextView {
  text: String,
  align: Align,
  wrap: Wrap,
}

impl Default for TextView {
  fn default() -> TextView {
    TextView {
      wrap: Wrap::Whitespace,
      text: String::from(""),
      align: Align::Start,
    }
  }
}
