//! Run with `cargo run --example hello_world` command.
//!
//! To connect through browser, navigate to "http://localhost:3000" url.
use chrono::{Local, Utc};
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/arnold", get(|| async { "He is habbool" }))
        .route(
            "/getlocaltime",
            get(|| async {
                let current_local = Local::now().to_string();
                format!("{current_local}")
            }),
        )
        .route("/getutctime", get(|| async {
            let current_utc = Utc::now().to_string();
            format!("{current_utc}")
        }));

    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
