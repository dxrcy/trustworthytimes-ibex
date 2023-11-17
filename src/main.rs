use ibex::prelude::*;
use ibex::{routes, ssg};
use news::Article;

mod news;

/// Name of github repo
const URL_ROOT: &str = "/trustworthytimes/";

fn main() {
    let articles = news::get_articles(ibex::is_local());

    let routes = routes! [
        (/)
            => at_index(&articles),
        (/404)
            => at_404(),
        (/[article.id])
            for article in articles
            => at_article(article),
    ];

    ssg::quick_build(routes).expect("Failed to build");
    println!("\x1b[34;1mBuilt successfully!\x1b[0m");
}

fn at_article(article: Article) -> Document {
    let Article {
        id: _,
        headline,
        desc,
        author,
        date,
        topic,
        image,
        alt,
        tags,
        body,
    } = article;
    view! {
        @use_base

        h1 { [headline] }
        h2 { [desc] }
        p { [author] }
        p { [date] }
        p {
            [:for (i, topic) in topic.into_iter().enumerate() {
                [:if i > 0 { ~ &gt; ~ }]
                [topic]
            }]
        }
        p {
            [:for (i, tag) in tags.into_iter().enumerate() {
                [:if i > 0 { "," ~ }]
                [tag]
            }]
        }
        img [alt=alt, src=image, width=200]/
        p { [body] }
    }
    .into()
}

fn at_index(articles: &[Article]) -> Document {
    view! {
        @use_base

        ul {
            [:for article in articles {
                li {
                    a [href=url!(article.id)] {
                        b { [&article.headline] }
                    }
                }
            }]
        }
    }
    .into()
}

fn at_404() -> Document {
    view! {
        @use_base

        center {
            "404 - Not found"
        }
    }
    .into()
}

fn use_base() -> View {
    view! {
        HEAD {
            @use_meta [Meta::new()]
            title { "Trustworthy Times" }
            link [rel="shortcut icon", href=url!("static/images/icon.png")]/
            link [rel="stylesheet", href=url!("css/base.css")]/
        }

        center {
            h1 {
                a [href=url!()] {
                    "The Trustworthy Times"
                }
            }
        }
    }
}
