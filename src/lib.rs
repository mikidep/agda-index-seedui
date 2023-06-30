// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{fetch, prelude::*, *};

mod model;
use model::{FetchError, Model, Reference};
mod view;
use view::view;
mod update;
use update::{update, Msg};
// ------ ------
//     Init
// ------ ------

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
        needle: "".into(),
        haystack: Ok(vec![]),
        frame_url: None
    }
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
