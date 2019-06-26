use ffi;
use std::ffi::CStr;

use crate::Parser;

/// Abstraction allowing the rendering of the abstract CommonMark document in several formats.

pub struct Render;

impl Render {
    /// Rendering the parser content in HTML format.
    pub fn to_html(parser: &Parser) -> String {
        let document = parser.finish();
        let extensions = parser.get_extensions();
        let options = parser.options.bits() as i32;

        unsafe {
            let result = ffi::cmark_render_html(document, options, extensions);

            ffi::cmark_node_free(document);

            CStr::from_ptr(result).to_string_lossy().into_owned()
        }
    }
}
