extern crate cmark_gfm;

use cmark_gfm::{Options, Parser, Render};

fn main() {
    let text = &"# Hello world";

    let mut options = Options::empty();
    options.insert(Options::CMARK_OPT_FOOTNOTES);

    let parser = Parser::new(options);

    let extensions = &["table", "strikethrough", "autolink", "tasklist"];
    for extension in extensions {
        parser.add_extension(extension).unwrap();
    }

    parser.parse(text);
    let result = Render::to_html(&parser);

    println!("Output : {}", result);
}
