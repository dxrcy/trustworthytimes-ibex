use ibex::prelude::*;
use ibex::{routes, ssg};

fn main() {
    let routes = routes! [
        (/) => at_index(),
    ];

    ssg::build(routes).unwrap();
}

fn at_index() -> Document {
    view! {
        h1 {
            "My Website"
        }
        p {
            "This is a template"
        }
    }
    .into()
}
