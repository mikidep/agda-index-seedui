use std::{iter::FromIterator, path::PathBuf};

use seed::{prelude::*, *};

use crate::{model::CountedReference, update::Msg, Model};

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["flex-container"],
        sidebar(&model.search_results),
        div![id!("content"), doc_view(&model.frame_url),]
    ]
}

fn search_item((idx, refr): &CountedReference) -> Node<Msg> {
    let p = PathBuf::from_iter(["static", "agda-html", &refr.href])
        .to_str()
        .unwrap_or_default()
        .to_string();

    li![
        el_key(idx),
        C!["indexentry"],
        a![
            mouse_ev(Ev::Click, |_| Msg::Display(p)),
            span![C!["name"], refr.identifier.clone()],
            span![C!["package", "dimmed"], "cubical"],
            br![],
            span![C!["namespace", "dimmed"], refr.module.clone()]
        ]
    ]
}

fn sidebar(search_results: &Vec<CountedReference>) -> Node<Msg> {
    div![
        id!("sidebar"),
        div![
            C!["searchbox-container"],
            input![
                attrs! {
                    At::Placeholder => "Search...",
                    At::Type => "text"
                },
                C!["searchbox"],
                id!("i2d_searchbox"),
                input_ev(Ev::Input, Msg::Search),
            ],
            span![C!["key-shortcut"], "Tab ⇥"],
        ],
        ul![
            id!("i2d_search_results"),
            search_results.iter().take(100).map(search_item)
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
        None => div!["No URL"],
    }
}
