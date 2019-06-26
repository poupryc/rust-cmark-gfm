extern crate cmark_gfm;

use cmark_gfm::{ Parser, Render, Options, markdown_to_html };

#[test]
fn simple_to_html() {
    let text = &"# Hello you !\nThis is a ~~ismple~~ simple test :)\nWe **test** things !";

    let options = Options::empty();
    let result = markdown_to_html(text, options);

    assert_eq!(result, "<h1>Hello you !</h1>\n<p>This is a ~~ismple~~ simple test :)\nWe <strong>test</strong> things !</p>\n");
}


#[test]
fn parser_to_html() {
    let options = Options::empty();
    let parser = Parser::new(options);

    let text = &"# Hello world !\nlorem ipsum _dolor_ **sit**";
    parser.parse(text);
    
    let result = Render::to_html(&parser);

    assert_eq!(result, "<h1>Hello world !</h1>\n<p>lorem ipsum <em>dolor</em> <strong>sit</strong></p>\n");
}

#[test]
fn reusing_parser() {
    let options = Options::empty();
    let parser = Parser::new(options);

    let first = &"This is **first**";
    parser.parse(first);

    let first_result = Render::to_html(&parser);
    assert_eq!(first_result, "<p>This is <strong>first</strong></p>\n");

    let second = &"This is _second_";
    parser.parse(second);

    let second_result = Render::to_html(&parser);
    assert_eq!(second_result, "<p>This is <em>second</em></p>\n");
}

#[test]
fn with_extensions() {
    let options = Options::empty();
    let parser = Parser::new(options);

    let text = &"# Hello world !\nlorem ipsum _dolor_ **sit**";
    parser.parse(text);

    assert!(parser.add_extension(&"strikethrough").is_ok(), "the extension should exist");
    assert!(parser.add_extension(&"lol").is_err(), "the extension shouldn't exist");
 }