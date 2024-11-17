//! Run with `cargo run --example hello_world` command.
//!
//! To connect through browser, navigate to "http://localhost:3000" url.
use axum::{routing::get, Json, Router};
use chrono::{Local, Utc};

#[derive(serde::Serialize)]
struct Time {
    local_time: String,
    utc_time: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/arnoob", get(|| async { "He is habbool" }))
        .route(
            "/localtime",
            get(|| async {
                let current_local = Local::now().to_string();
                format!("{current_local}")
            }),
        )
        .route(
            "/utctime",
            get(|| async {
                let current_utc = Utc::now().to_string();
                format!("{current_utc}")
            }),
        )
        .route(
            "/time",
            get(|| async {
                let current_local = Local::now().to_string();
                let current_utc = Utc::now().to_string();            
                let t = Time {
                    local_time: current_local,
                    utc_time: current_utc,
                };
                Json(t)
            }),
        );

    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
