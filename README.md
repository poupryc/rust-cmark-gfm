<p align="center">
  <img src="https://raw.githubusercontent.com/helloedit/rust-cmark-gfm/master/media/markdown.png" />
</p>

<h2 align="center">Simple cmark-gfm Rust wrapper</h2>


This crates has been developed to cover only the main functions of `cmark-gfm`. Feel free to propose PR!

### Example

```rust
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
```

### License

MIT