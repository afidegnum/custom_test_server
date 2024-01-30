use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::MyObj;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    obj: MyObj,
    req: String,
}

fn _index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "Hello World!" }
        a(href = "about", id = "about-link") { "About!" }
    }
}

fn index_page<'a, G: Html>(
    cx: BoundedScope<'_, 'a>,
    IndexPageStateRx { obj, req }: &'a IndexPageStateRx,
) -> View<G> {
    // If the future hasn't finished yet, we'll display a placeholder
    // We use the wacky `&*` syntax to get the content of the `browser_ip` `Signal`
    // and then we tell Rust to take a reference to that (we can't move it out
    // because it might be used later)

    view! { cx,
        p { (format!("The Object: {:#?}", obj.get())) }
        p { (format!("The Req: {:#?}", req.get())) }
    }
}

#[engine_only_fn]
async fn get_request_state(
    // We get all the same info as build state in here
    _info: StateGeneratorInfo<()>,
    // Unlike in build state, in request state we *also* get access to the information that the
    // user sent with their HTTP request IN this example, we extract the browser's reporting of
    // their IP address and display it to them
    req: Request,
) -> Result<IndexPageState, BlamedError<std::convert::Infallible>> {
    Ok(IndexPageState {
        req: format!("{:#?}", req),
        obj: MyObj {
            name: "".to_owned(),
            number: 0,
        },
    })
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
            let res = reqwest::get("http://127.0.0.1:8080/hello")
                .await?
                .text()
                .await?;

            let item = MyObj::default();
            Ok::<MyObj, reqwest::Error>(item)
        },
        false,
    )
    .await?; // Note that `?` is able to convert from `reqwest::Error` ->
             // `BlamedError<reqwest::Error>`

    Ok(IndexPageState {
        obj: body,
        req: "".to_owned(),
    })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .request_state_fn(get_request_state)
        .build()
}
