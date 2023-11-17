use crate::URL_ROOT;
use ibex::url;
use std::{fs, io};

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
    pub fn from(
        id: impl Into<String>,
        content: impl Into<String>,
        url: &str,
    ) -> Result<Article, ()> {
        Ok(Article {
            id: id.into(),
            headline: "Foo".to_owned(),
            desc: "Bar".to_owned(),
            author: "John Smith".to_owned(),
            date: "1970-01-01".to_owned(),
            topic: vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()],
            image: "#".to_owned(),
            alt: "Non-existant image".to_owned(),
            tags: vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()],
            body: "This is an example article".to_owned(),
        })
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
        let article = Article::from(id, content, &url!()).expect("Failed to parse article");
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
