//! Generate html for webpages, blogs, and documentation for libs & schemas.

use pulldown_cmark::{Parser, Options, html};

fn add_links(input: &str) -> String {
    let mut output = String::new();
    for line in input.lines() {
        let words = line.split(' ');
        for word in words {
            let is_email = word.contains('@') && word.contains('.');
            let is_link = word.starts_with("http://") || is_email;
            if is_link {
                output.push('[');
            }
            output.push_str(word);
            if is_link {
                output.push_str("](");
                output.push_str(if is_email {
                    "mailto:"
                } else {
                    "https://"
                });
                output.push_str(word);
                output.push(')');
            }
            output.push(' ');
        }
        output.push('\n');
    }
    output
}

fn gen_page(markdown_filename: &str) {
    let markdown_filename = std::path::Path::new(markdown_filename);

    println!("Generating Page {}…", markdown_filename.display());

    let markdown_input = &if let Ok(input) = std::fs::read_to_string(markdown_filename) {
        input
    } else {
        eprintln!("Couldn't read file '{}'?", markdown_filename.display());
        return;
    };
    let markdown_input = &add_links(markdown_input);
    let parser = Parser::new_ext(markdown_input, Options::all());
     
    let mut page = r#"<!DOCTYPE html><html><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>"#.to_string();
    if markdown_input.starts_with('#') {
        if let Some(offset) = markdown_input.find('\n') {
            let title = markdown_input.split_at(offset).0[1..].trim_start();
            page.push_str(title);
            page.push_str(" | ");
        }
    }
//    page.push_str(&context.page_title);
    page.push_str("</title>");
//    page.push_str(&context.head);
    page.push_str("</head><body>");
//    page.push_str(&context.body);
    html::push_html(&mut page, parser);
//    page.push_str(&context.foot);

    if let Some(stem) = markdown_filename.file_stem() {
        let mut path = "".to_string();
        path.push_str(&stem.to_string_lossy());
        path.push_str(".html");
        let _ = std::fs::write(path, page);
    } else {
        eprintln!("ERROR: No available stem!");
    }

    println!("Generated Page {}…", markdown_filename.display());
}

fn main() {
    gen_page("README.md");
}
