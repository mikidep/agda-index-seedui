use std::{path::PathBuf, iter::FromIterator};

use seed::{prelude::*, *};

use crate::{model::{Model, Reference, HaystackError}, update::Msg};

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["flex-container"],
        sidebar(&model.needle, &model.haystack),
        div![id!("content"), doc_view(&model.frame_url),]
    ]
}

fn search_item(refr: &Reference) -> Node<Msg> {
    let p = PathBuf::from_iter(["static", "agda-html", &refr.href])
        .to_str().unwrap_or_default().to_string();
                        
    li![
        C!["indexentry"],
        a![
            mouse_ev(
                Ev::Click, |_| Msg::Display(p)
            ),
            span![C!["name"], refr.identifier.clone()],
            span![C!["package", "dimmed"], "cubical"],
            br![],
            span![C!["namespace", "dimmed"], refr.module.clone()]
        ]
    ]
}

fn sidebar(needle: &str, haystack: &Result<Vec<Reference>, HaystackError>) -> Node<Msg> {
    div![
        id!("sidebar"),
        div![
            C!["searchbox-container"],
            input![
                attrs! {
                    At::Placeholder => "Search...",
                    At::Value => needle,
                    At::Type => "text"
                },
                C!["searchbox"],
                id!("i2d_searchbox"),
                input_ev(Ev::Input, Msg::Search),
            ],
            span![
                C!["key-shortcut"],
                "Tab ⇥"
            ],
        ],
        ul![
            id!("i2d_search_results"),
            match haystack {
                Ok(haystack) => haystack.iter().map(|refr| {
                    search_item(refr)
                }).collect(),
                Err(_) => vec![li!["Error".to_owned()]],
            }
        ]
    ]
}



fn doc_view(url: &Option<String>) -> Node<Msg> {
    match url {
        Some(url) => iframe![
            style! {
                St::Width => "100%",
                St::Height => "100%"
            },
            attrs! {
                At::Src => url
            }
        ],
        None => div!["No URL"]
    }
}