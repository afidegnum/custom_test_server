use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::MyObj;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    obj: String,
}

fn _index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "Hello World!" }
        a(href = "about", id = "about-link") { "About!" }
    }
}

fn index_page<'a, G: Html>(
    cx: BoundedScope<'_, 'a>,
    IndexPageStateRx { obj }: &'a IndexPageStateRx,
) -> View<G> {
    // If the future hasn't finished yet, we'll display a placeholder
    // We use the wacky `&*` syntax to get the content of the `browser_ip` `Signal`
    // and then we tell Rust to take a reference to that (we can't move it out
    // because it might be used later)

    view! { cx,
        p { (format!("IP address of the server was: {:?}", obj.get())) }
    }
}

#[engine_only_fn]
async fn get_build_state(
    _info: StateGeneratorInfo<()>,
) -> Result<IndexPageState, BlamedError<reqwest::Error>> {
    // We'll cache the result with `try_cache_res`, which means we only make the
    // request once, and future builds will use the cached result (speeds up
    // development)
    let body = perseus::utils::cache_fallible_res(
        "ipify",
        || async {
            // This just gets the IP address of the machine that built the app
            let res = reqwest::get("http://0.0.0.0:8080/.perseus/hello")
                .await?
                .text()
                .await?;

            let item = MyObj::default();
            Ok::<String, reqwest::Error>(res)
        },
        false,
    )
    .await?; // Note that `?` is able to convert from `reqwest::Error` ->
             // `BlamedError<reqwest::Error>`

    Ok(IndexPageState { obj: body })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .build()
}
