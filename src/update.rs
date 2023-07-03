use seed::prelude::*;

use crate::{model::{Model, Reference, FetchError}, search::SearchEngine};

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    Search(String),
    Fetched(Result<Vec<Reference>, FetchError>),
    Display(String)
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Search(needle) => {
            model.search_results = model.search_engine.search(&needle);
        },
        Msg::Fetched(Ok(response)) => {
            model.search_engine = SearchEngine::new(response);
            model.search_results = model.search_engine.haystack.clone();
        },
        Msg::Fetched(Err(_)) => (),
        Msg::Display(url) => {
            model.frame_url = Some(url);
        }
    }
}
