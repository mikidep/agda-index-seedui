use seed::prelude::*;

use crate::{model::{Model, Reference, FetchError}, search::SearchEngine, time};

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    Search(String),
    Fetched(Result<Vec<Reference>, FetchError>),
    Display(String)
}

// `update` describes how to handle each `Msg`.
pub fn update<SE: SearchEngine>(msg: Msg, model: &mut Model<SE>, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Search(needle) => {
            model.search_results = time![model.search_engine.search(&needle)];
        },
        Msg::Fetched(Ok(response)) => {
            model.search_engine = time![SE::new(response)];
            model.search_results = model.search_engine.all();
        },
        Msg::Fetched(Err(_)) => (),
        Msg::Display(url) => {
            model.frame_url = Some(url);
        }
    }
}
