mod paths;
mod structs;
mod utils;

use axum::{routing::get, Router};
use dotenv::dotenv;
use hyper::{header, Method};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // Load .env variables
    dotenv().ok();
    let sentry_dns = env::var("SENTRY_DNS").unwrap_or_else(|_| {
        eprintln!("SENTRY_DNS not found in .env file.");
        "".to_string()
    });
    let _guard = sentry::init((
        sentry_dns,
        sentry::ClientOptions {
            traces_sample_rate: 0.0,
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));
    // initialize tracing
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET])
        // pay attention that for some request types like posting content-type: application/json
        // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
        // or see this issue https://github.com/tokio-rs/axum/issues/849
        .allow_headers([header::CONTENT_TYPE])
        // allow requests from any origin
        .allow_origin(Any);

    // build our application with a route
    let app = Router::new()
        .route("/", get(|| async { "CPF Generator and Validator API. For more information see https://github.com/OLoKo64/rust-cpf-generator-api/tree/main/axum" }))
        .route("/validate-cpf", get(paths::validate_cpf))
        .route("/gen-cpf", get(paths::new_cpf))
        // For more information about layer see https://docs.rs/axum/0.6.0-rc.1/axum/struct.Router.html#method.layer
        .layer(cors)
        // TraceLayer provides good defaults but is also very customizable.
        // See https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html for more details.
        .layer(TraceLayer::new_for_http());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        env::var("PORT")
            .unwrap_or_else(|_| {
                println!("PORT not found .env file, using default port: 3000");
                "3000".to_string()
            })
            .parse::<u16>()
            .expect("Failed to parse port from .env file"),
    ));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server error");
}
