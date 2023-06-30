use seed::{prelude::*, *};

use crate::model::{Model, Reference, FetchError, HaystackError};

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
            model.needle = needle;
            todo!("Search for `{}`", model.needle)
        },
        Msg::Fetched(Ok(response)) => {
            model.haystack = Ok(response);
        },
        Msg::Fetched(Err(_)) => {
            model.haystack = Err(HaystackError::FetchError);
        },
        Msg::Display(url) => {
            model.frame_url = Some(url);
        }
    }
}
