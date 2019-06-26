extern crate bitflags;
use ffi;
use std::ffi::{CStr, CString};

bitflags! {
    /// Options that can be passed to the parser, defined by libcmark-gfm.
    pub struct Options: u32 {
        const CMARK_OPT_DEFAULT = ffi::CMARK_OPT_DEFAULT;
        const CMARK_OPT_SOURCEPOS = ffi::CMARK_OPT_SOURCEPOS;
        const CMARK_OPT_HARDBREAKS = ffi::CMARK_OPT_HARDBREAKS;
        const CMARK_OPT_SAFE = ffi::CMARK_OPT_SAFE;
        const CMARK_OPT_UNSAFE = ffi::CMARK_OPT_UNSAFE;
        const CMARK_OPT_NOBREAKS = ffi::CMARK_OPT_NOBREAKS;
        const CMARK_OPT_NORMALIZE = ffi::CMARK_OPT_NORMALIZE;
        const CMARK_OPT_VALIDATE_UTF8 = ffi::CMARK_OPT_VALIDATE_UTF8;
        const CMARK_OPT_SMART = ffi::CMARK_OPT_SMART;
        const CMARK_OPT_GITHUB_PRE_LANG = ffi::CMARK_OPT_GITHUB_PRE_LANG;
        const CMARK_OPT_LIBERAL_HTML_TAG = ffi::CMARK_OPT_LIBERAL_HTML_TAG;
        const CMARK_OPT_FOOTNOTES = ffi::CMARK_OPT_FOOTNOTES;
        const CMARK_OPT_STRIKETHROUGH_DOUBLE_TILDE = ffi::CMARK_OPT_STRIKETHROUGH_DOUBLE_TILDE;
        const CMARK_OPT_TABLE_PREFER_STYLE_ATTRIBUTES = ffi::CMARK_OPT_TABLE_PREFER_STYLE_ATTRIBUTES;
        const CMARK_OPT_FULL_INFO_STRING = ffi::CMARK_OPT_FULL_INFO_STRING;
    }
}

/// An abstraction of the `libcmark-gfm` parser, a CommonMark parser 
/// supporting GitHub Flavored Markdown, using `libcmark-gfm` bindings.

pub struct Parser {
    parser: *mut ffi::cmark_parser,
    pub options: Options,
}

impl Parser {
    /// Creates a new cmark-gfm's parser abstraction with the defined options.
    pub fn new(options: Options) -> Parser {
        unsafe { 
            ffi::cmark_gfm_core_extensions_ensure_registered(); 
        };

        let parser = unsafe { ffi::cmark_parser_new(options.bits() as i32) };

        Parser { parser, options }
    }

    /// Feeds a string to parser.
    pub fn parse(&self, content: &str) {
        let content = CString::new(content).unwrap();

        unsafe {
            ffi::cmark_parser_feed(self.parser, content.as_ptr(), content.as_bytes().len());
        };
    }

    /// Returns a list containing the extensions of the current parser.
    pub fn get_extensions(&self) -> *mut ffi::_cmark_llist {
        unsafe { ffi::cmark_parser_get_syntax_extensions(self.parser) }
    }

    /// Finish parsing and return a pointer to a tree of nodes.
    pub fn finish(&self) -> *mut ffi::cmark_node {
        unsafe { ffi::cmark_parser_finish(self.parser) }
    }

    pub fn add_extension(&self, name: &str) -> Result<(), String> {
        let extension = CString::new(name).unwrap();

        unsafe {
            let syntax_extension = ffi::cmark_find_syntax_extension(extension.as_ptr());

            if syntax_extension.is_null() {
                return Err(format!("Unknown extension name `{}`", name));
            }

            let result = ffi::cmark_parser_attach_syntax_extension(self.parser, syntax_extension);
            if result == 0 {
                return Err(format!("Unable to attach `{}` extension", name));
            }

            Ok(())
        }
    }
}

impl Drop for Parser {
    fn drop(&mut self) {
        unsafe {
            ffi::cmark_parser_free(self.parser);
        };
    }
}

/// Convert "content" (assumed to be a UTF-8 encoded string) from CommonMark Markdown to HTML.

pub fn markdown_to_html(content: &str, options: Options) -> String {
    let content = CString::new(content).unwrap();
    let options = options.bits() as i32;

    let result = unsafe {
        let ptr = ffi::cmark_markdown_to_html(content.as_ptr(), content.as_bytes().len(), options);
        CStr::from_ptr(ptr)
    };

    result.to_string_lossy().into_owned()
}
