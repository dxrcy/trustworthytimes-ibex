use ibex::prelude::*;
use ibex::{routes, ssg};

const URL_ROOT: &str = "/ibex-template/";

fn main() {
    let secret = "Hello!";
    let routes = routes! [
        (/) => at_index(secret),
    ];

    ssg::quick_build(routes).unwrap();
}

fn at_index(secret: &str) -> Document {
    view! {
        @use_basic[]

        center {
            h1 { "My Ibex App" }
            p {
                "This app was made using"
                ~ a [href="https://github.com/darccyy/ibex-template"] {
                    "Ibex Template"
                }
            }
            p {
                "Secret message:" ~ i { "'" [secret] "'" }
            }
            p {
                a [href=url!("not/a/real/path")] {
                    "404 Example"
                }
            }
            br/
            img [
                alt="Ibex logo",
                src=url!("static/icon.png"),
                width=300,
            ]/
        }
    }
    .into()
}

fn use_basic() -> View {
    view! {
        HEAD {
            title { "My Ibex App" }
            link [rel="shortcut icon", href=url!("static/icon.png")]/
            link [rel="stylesheet", href=url!("css/main.css")]/
        }
    }
}
