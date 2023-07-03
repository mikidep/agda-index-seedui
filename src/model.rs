// ------ ------
//     Model
// ------ ------

use std::{fmt::Display, error::Error};

use serde::Deserialize;

use crate::search::SearchEngine;

#[derive(Debug, PartialEq)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParseError")
    }
}

impl Error for ParseError {}

#[derive(Debug, PartialEq)]
pub struct FetchError;

impl Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FetchError")
    }
}

impl Error for FetchError {}

#[derive(Debug, PartialEq)]
pub enum HaystackError {
    _ParseError,
    _FetchError,
}

impl Display for HaystackError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HaystackError::_ParseError => write!(f, "ParseError"),
            HaystackError::_FetchError => write!(f, "FetchError"),
        }
    }
}

impl Error for HaystackError {}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Reference {
    pub id : String,
    pub identifier : String,
    pub qualified : String,
    pub module : String,
    pub href : String,
    pub content : String,
}

// `Model` describes our app state.
pub struct Model {
    pub frame_url: Option<String>,
    pub search_engine: SearchEngine,
    pub search_results: Vec<Reference>,
}
