#[macro_use]
extern crate bitflags;
extern crate cmark_gfm_sys as ffi;

mod parser;
mod render;

pub use crate::parser::{markdown_to_html, Options, Parser};
pub use crate::render::Render;
