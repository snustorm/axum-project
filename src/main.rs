#![allow(unused)]

use std::net::SocketAddr;
use axum::{extract::{Path, Query}, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Router};
use model::ModelController;
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_cookies::{CookieManager, CookieManagerLayer};
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};

mod error;
mod web;
mod model;
mod ctx;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {

    // Initialize the ModelController.

    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routers(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .nest("/api", web::routes_tickets::routers(mc.clone()))
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static());

    //region:  --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTERNING on {addr}\n");
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    //endregion: --- Start Server

    Ok(())

}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();

    res

} 


fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region: --- Routes Hello

fn routes_hello() -> Router {
    Router::new()
    .route("/hello", get(handler_hello))
    .route("/hello2/:name", get(handler_hello2))
}

async fn handler_hello(params: Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?} ", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");

    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g., `/hello2/Mike`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {name:?} ", "HANDLER");
    Html(format!("Hello <strong>{name}</strong>"))
}


// endregion            --- Handler Hello