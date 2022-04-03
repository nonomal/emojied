mod components;
mod config;
mod controllers;
pub mod db;
mod emoji;
mod views;

use axum::extract::Extension;
use axum::handler::Handler;
use axum::routing;
use axum::Router;
use std::sync::Arc;

pub async fn run(handle: db::Handle) -> Result<(), hyper::Error> {
    // https://docs.rs/axum/0.4.8/axum/extract/struct.Extension.html
    // TODO: Read about `Arc` because I have no idea what this does.
    let app_handle = Arc::new(handle);

    let app = Router::new()
        .fallback(controllers::not_found.into_service())
        .route("/", routing::get(controllers::root))
        .route("/", routing::post(controllers::insert_url))
        .route(
            "/rpc/shorten-url",
            routing::post(controllers::rpc_insert_url),
        )
        .route("/leaderboard", routing::get(controllers::leaderboard))
        .route("/stats/:id", routing::get(controllers::url_stats))
        .route("/:id", routing::get(controllers::fetch_url))
        .route("/assets/app.css", routing::get(controllers::assets::stylesheet))
        .route("/assets/app.js", routing::get(controllers::assets::js))
        .route("/assets/purify.min.js", routing::get(controllers::assets::purifyjs))
        .route("/assets/icons/:id", routing::get(controllers::assets::favico))
        .layer(Extension(app_handle));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(signal_shutdown())
        .await
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn signal_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}
