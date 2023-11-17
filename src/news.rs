use crate::URL_ROOT;
use ibex::url;
use std::{collections::HashMap, fs, io};

#[derive(Debug)]
pub struct Article {
    pub id: String,
    pub headline: String,
    pub desc: String,
    pub author: String,
    pub date: String,
    pub topic: Vec<String>,
    pub image: String,
    pub alt: String,
    pub tags: Vec<String>,
    pub body: String,
}

impl Article {
    pub fn from(id: impl Into<String>, content: &str, url: &str) -> Result<Article, ()> {
        let id = id.into();
        let (mut body, meta) = parse_news(content);

        for (key, value) in &meta {
            body = body.replace(&format!("@{key}"), value);
        }
        body = format_primative_styles(&body);
        body = format_links(&body, url);

        macro_rules! meta {
            ( $key:literal ) => {
                meta.get($key)
                    .expect(concat!("Missing `", $key, "`"))
                    .to_owned()
            };
        }

        // Lists
        let topic = split_meta_list(meta!("topic"));
        let tags = meta.get("tags").map(split_meta_list).unwrap_or_default();

        let image = meta!("image");
        let image = if image == "@" {
            // Use image from id
            format!("{url}static/thumbs/{id}.jpg")
        } else if image.starts_with('@') {
            // Use custom path relative to url
            // Remove first character
            let mut chars = image.chars();
            chars.next();
            format!("{url}{}", chars.as_str())
        } else {
            // Full url
            image
        };

        Ok(Article {
            id,
            headline: meta!("headline"),
            desc: meta!("desc"),
            author: meta!("author"),
            date: meta!("date"),
            topic,
            image,
            alt: meta!("alt"),
            tags,
            body,
        })
    }
}

fn format_links(body: &str, url: &str) -> String {
    let mut output = String::new();
    let mut current_link = None::<String>;

    for ch in body.chars() {
        match &mut current_link {
            Some(link) => {
                if ch == ']' {
                    if let Some(link) = convert_link_to_html(link, url) {
                        output += &link;
                    }
                    current_link = None;
                } else {
                    link.push(ch);
                }
            }
            None => {
                if ch == '[' {
                    current_link = Some(String::new());
                } else {
                    output.push(ch);
                }
            }
        }
    }

    output
}

fn convert_link_to_html(link: &str, url: &str) -> Option<String> {
    let mut split = link.split('|');

    let href = split.next_back()?;
    let href = replace_with_url(href, url);

    let desc = split.collect::<Vec<_>>().join("|");
    let desc = desc.trim();
    let desc = if desc.is_empty() { &href } else { desc };

    Some(format!("<a href={href}>{desc}</a>"))
}

/// Replace `@` at start of link with url
fn replace_with_url(link: &str, url: &str) -> String {
    if link.starts_with('@') {
        let mut chars = link.chars();
        chars.next();
        format!("{url}{}", chars.as_str())
    } else {
        link.to_string()
    }
}

fn format_primative_styles(body: &str) -> String {
    #[derive(Default)]
    struct Primatives {
        italic: bool,
        bold: bool,
        underline: bool,
        strike: bool,
        code: bool,
        purple: bool,
    }
    let mut prims = Primatives::default();

    let mut output = String::new();
    let mut is_escaped = false;

    for ch in body.chars() {
        if is_escaped {
            output.push(ch);
            is_escaped = false;
            continue;
        }

        macro_rules! toggle_style {
            ( $name:ident $tag:literal ) => {{
                output += if prims.$name {
                    concat!("</", $tag, ">")
                } else {
                    concat!("<", $tag, ">")
                };
                prims.$name ^= true;
            }};
        }

        match ch {
            '\\' => is_escaped = true,
            '*' => toggle_style!(italic "i"),
            '^' => toggle_style!(bold "b"),
            '_' => toggle_style!(underline "u"),
            '~' => toggle_style!(strike "strike"),
            '`' => toggle_style!(code "code"),
            '%' => {
                output.push_str(if prims.purple {
                    "</span>"
                } else {
                    r#"<span class="purple">"#
                });
                prims.purple ^= true;
            }
            _ => output.push(ch),
        }
    }

    output
}

fn split_meta_list(line: impl AsRef<str>) -> Vec<String> {
    line.as_ref()
        .split('|')
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.trim().to_string())
        .collect()
}

type Dict = HashMap<String, String>;

fn parse_news(file: &str) -> (String, Dict) {
    enum ListKind {
        None,
        Unordered,
        Ordered,
    }

    let mut body: Vec<String> = Vec::new();
    let mut meta = Dict::new();

    let mut is_meta = true;
    let mut current_list = ListKind::None;

    for line in file.lines() {
        let line = line.trim();
        let (token, rest) = take_first_word(line);
        let rest = rest.trim();

        if is_meta {
            match token {
                _ if token.starts_with('@') => {
                    meta.insert(token.split_at(1).1.to_string(), rest.to_string());
                }
                "#" => {
                    meta.insert("headline".to_string(), rest.to_string());
                }
                "##" => {
                    meta.insert("desc".to_string(), rest.to_string());
                }
                "---" => is_meta = false,
                _ => (),
            }
        } else {
            let line_push = match token {
                _ if is_header_token(token) => {
                    format!("<h{d}>{}</h{d}>", sanitize_html(rest), d = token.len() + 1)
                }

                ">" => format!("<blockquote>{}</blockquote>", sanitize_html(rest)),

                "---" => "<hr/>".to_string(),

                "-" => {
                    let parent = match current_list {
                        ListKind::None => "<ul>",
                        ListKind::Ordered => "</ol>\n<ul>\n",
                        ListKind::Unordered => "",
                    };
                    current_list = ListKind::Unordered;
                    format!("{parent}<li>{}</li>", sanitize_html(rest))
                }

                _ if is_ordered_list_token(token) => {
                    let parent = match current_list {
                        ListKind::None => "<ol>",
                        ListKind::Ordered => "",
                        ListKind::Unordered => "</ul>\n<ol>\n",
                    };
                    current_list = ListKind::Ordered;
                    format!("{parent}<li>{}</li>", sanitize_html(rest))
                }

                "~~~" => continue,

                _ => {
                    if line.is_empty() {
                        continue;
                    }
                    format!("<p>{}</p>", sanitize_html(line))
                }
            };
            body.push(line_push);
        }
    }

    (body.join("\n"), meta)
}

fn sanitize_html(dirty: &str) -> String {
    dirty
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn is_header_token(token: &str) -> bool {
    token.chars().all(|ch| ch == '#')
}

fn is_ordered_list_token(token: &str) -> bool {
    let mut chars = token.chars();
    if chars.next_back() != Some('.') {
        return false;
    }
    chars.as_str().parse::<u32>().is_ok()
}

fn take_first_word(string: &str) -> (&str, &str) {
    match string.find(' ') {
        Some(pos) => string.split_at(pos),
        None => (string, ""),
    }
}

pub fn get_articles(include_test_files: bool) -> Vec<Article> {
    const NEWS_DIR: &str = "news";

    let mut articles = Vec::new();
    let files = read_all_files(NEWS_DIR).expect("Failed to read news files");
    for (filename, content) in files {
        let id = remove_file_extension(&filename);
        if id.ends_with(".test") && !include_test_files {
            continue;
        }
        println!("{}", id);
        let article = Article::from(id, &content, &url!()).expect("Failed to parse article");
        articles.push(article);
    }

    articles
}

fn read_all_files(dir: &str) -> io::Result<Vec<(String, String)>> {
    let mut files = Vec::new();

    let children = fs::read_dir(dir)?;
    for child in children {
        let child = child?;

        let path = child.path();
        if !path.is_file() {
            continue;
        }

        let filename = child.file_name();
        let filename = filename.to_string_lossy().to_string();

        let content = fs::read_to_string(path)?;

        files.push((filename, content));
    }

    Ok(files)
}

fn remove_file_extension(filename: &str) -> &str {
    let mut chars = filename.chars();
    loop {
        match chars.next_back() {
            Some('.') | None => break,
            _ => (),
        }
    }
    let new_filename = chars.as_str();
    if new_filename.is_empty() {
        filename
    } else {
        new_filename
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_file_extension_works() {
        assert_eq!(remove_file_extension(""), "");
        assert_eq!(remove_file_extension("foo"), "foo");
        assert_eq!(remove_file_extension("foo.bar"), "foo");
        assert_eq!(remove_file_extension("foo.bar.baz"), "foo.bar");
    }
}
