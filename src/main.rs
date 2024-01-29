mod templates;

use perseus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(engine)]
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};

#[derive(Debug, Serialize, Clone, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

#[cfg(engine)]
impl Default for MyObj {
    fn default() -> Self {
        Self {
            name: "Default Name".to_string(),
            number: 123243,
        }
    }
}

#[cfg(engine)]
#[get("/home")]
async fn index() -> HttpResponse {
    let item = MyObj::default();
    println!("model: {:?}", &item.number);
    HttpResponse::Ok().json(item) // <- send response
}

#[cfg(engine)]
#[get("/hello")]
async fn hello(req: HttpRequest) -> HttpResponse {
    let r = format!("REQ: {req:?}");
    HttpResponse::Ok().json(r) // <- send response
}

#[cfg(engine)]
pub async fn dflt_server<
    M: perseus::stores::MutableStore + 'static,
    T: perseus::i18n::TranslationsManager + 'static,
>(
    turbine: &'static perseus::turbine::Turbine<M, T>,
    opts: perseus::server::ServerOptions,
    (host, port): (String, u16),
) {
    use futures::{executor::block_on, Future};
    use perseus_actix_web::configurer;
    use std::time::Duration;

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(hello)
            .configure(block_on(configurer(turbine, opts.clone())))
    })
    .bind((host, port))
    .expect(
        "Couldn't bind to given address. Maybe something is already running on the selected port?",
    )
    .run()
    .await
    .expect("Server failed.")
}

#[perseus::main(dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::about::get_template())
        .error_views(ErrorViews::unlocalized_development_default())
}
