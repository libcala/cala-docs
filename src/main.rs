//! Generate html for webpages, blogs, and documentation for libs & schemas.

use comrak::{markdown_to_html, ComrakOptions};

fn add_links(input: &str, name: &str) -> String {
    let mut output = String::new();
    let mut code = false;
    for line in input.lines() {
        let mut words = line.split(' ');
        if let Some(word) = words.next() {
            output.push_str(word);
            output.push(' ');
            if word.starts_with("```") {
                code = !code;
            }
            if word.contains('#') && !code {
                let mut link_to = if name == "README" { "" } else { name }.to_string();
                link_to.push('#');
                output.push_str("[");
                for word in words {
//                    let word = word.replace('&', "and");
                    output.push_str(&word);
                    output.push(' ');
                    let word = word.replace('&', "");
                    let word = word.replace('[', "");
                    let word = word.replace(']', "");
                    let word = word.replace('{', "");
                    let word = word.replace('}', "");
                    let word = word.replace('.', "");
                    let word = word.replace(',', "");
                    link_to.push_str(&word.to_lowercase());
                    link_to.push('-');
                }
                output.pop();
                link_to.pop();
                output.push_str("](");
                output.push_str(&link_to);
                output.push(')');
            } else {
                for word in words {
                    output.push_str(word);
                    output.push(' ');
                }
                output.pop();
            }
        }

/*        for word in words {
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
        }*/
        output.push('\n');
    }
    output
}

fn gen_page(markdown_filename: &str, menu: &str, options: &ComrakOptions) {
    let markdown_filename = std::path::Path::new(markdown_filename);

    let name = if let Some(stem) = markdown_filename.file_stem() {
        stem.to_string_lossy()
    } else {
        eprintln!("ERROR: No available stem!");
        return;
    };

    println!("Generating Page {}…", markdown_filename.display());

    let md_input = &if let Ok(input) = std::fs::read_to_string(markdown_filename) {
        input
    } else {
        eprintln!("Couldn't read file '{}'?", markdown_filename.display());
        return;
    };

    println!("BEFORE:\n{}\n\n", md_input);

    let markdown_input = &add_links(md_input, &name);

    println!("AFTER:\n{}\n\n", markdown_input);

    let menu =  &add_links(menu, &name);
     
    let mut page = r#"<!DOCTYPE html><html><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>"#.to_string();
    if let Some(start) = md_input.find("# ") {
        if let Some(offset) = md_input[start..].find('\n') {
            let title = md_input.split_at(offset).0[2..].trim_start();
            page.push_str(title);
        }
    }
    page.push_str("</title>");
    if !markdown_filename.ends_with("README.md") {
        page.push_str("<base href=\"../\">");
    }
    page.push_str("<link rel=\"icon\" href=\"icon.svg\"><link rel=\"stylesheet\" href=\"style.css\"/>");
    page.push_str("</head><body><div class=\"menu\">");
    page.push_str(&markdown_to_html(menu, options));
//    html::push_html(&mut page, menu_parser);
    page.push_str("</div><div class=\"content\"><div class=\"page\">");
    page.push_str(&markdown_to_html(markdown_input, options));
//    html::push_html(&mut page, parser);
    page.push_str("</div></div></body>");

    let mut path = "docs/".to_string();
    if name != "README" {
        path.push_str(&name);
        path.push_str("/");
        let dir = format!("docs/{}", name);
        if !std::path::Path::new(&dir).exists() {
            std::fs::create_dir(dir).unwrap();
        }
    }
    path.push_str("index.html");
    let _ = std::fs::write(path, page);

    println!("Generated Page {}…", markdown_filename.display());
}

fn main() {
    if !std::path::Path::new("docs").exists() {
        std::fs::create_dir("docs").unwrap();
    }

    let options = ComrakOptions {
    hardbreaks: true,
    smart: true,
    github_pre_lang: true,
    width: 80,
    default_info_string: Some("rust".into()),
    unsafe_: true,
    ext_strikethrough: true,
    ext_tagfilter: true,
    ext_table: true,
    ext_autolink: true,
    ext_tasklist: true,
    ext_superscript: true,
    ext_header_ids: Some("".into()),
    ext_footnotes: true,
    ext_description_lists: true,
};

    if !std::path::Path::new("style.css").exists() {
        let style: &'static [u8] = include_bytes!("style.css");
        let _ = std::fs::write("docs/style.css", style);
    }
    let menu = if let Ok(input) = std::fs::read_to_string("MENU.md") {
        input
    } else {
        "".to_string()
    };

    let paths = std::fs::read_dir("./").unwrap();

    for path in paths {
        let string = path.unwrap().path().to_str().unwrap().to_string();
        if string.ends_with(".md") {
            gen_page(&string, &menu, &options);
        }
    }
}
