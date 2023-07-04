// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use search::SearchEngine;
use seed::prelude::*;

mod model;
use model::{FetchError, Reference};
mod view;
use view::view;
mod update;
use update::{update, Msg};
mod search;
// ------ ------
//     Init
// ------ ------

type SE = search::probly::ProblySearchEngine;
pub type Model = model::Model<SE>;

async fn fetch_db() -> Result<Vec<Reference>, FetchError> {
    let res = fetch("/static/ingestible.json")
        .await.map_err(|_| FetchError)?;
    let ret: Vec<Reference> = res.json()
        .await.map_err(|_| FetchError)?;
    Ok(ret)
}

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::Fetched(fetch_db().await) });
    Model {
        frame_url: None,
        search_engine: SearchEngine::new(vec![]),
        search_results: vec![],
    }
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}

#[macro_export]
macro_rules! time {
    ($it: expr) => {
        {
            web_sys::console::time_with_label(stringify!($it));
            let ret = $it;
            web_sys::console::time_end_with_label(stringify!($it));
            ret
        }
    };
}

#[macro_export]
macro_rules! dbg {
    ($it: expr) => {
        {
            log::debug!("{}: {:?}", stringify!($it), $it);
            $it
        }
    };
}