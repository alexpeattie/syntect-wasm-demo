mod utils;

use syntect::highlighting::{Color, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn render(code: &str) -> String {
  let ss = SyntaxSet::load_defaults_newlines();
  let ts = ThemeSet::load_defaults();

  let theme = &ts.themes["base16-ocean.dark"];
  let sr = ss.find_syntax_by_extension("js").unwrap();

  highlighted_html_for_string(code, &ss, &sr, theme)
}
