use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let app = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());

    println!("->> LISTENING ON {addr}\n");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn routes_static() -> Router {
    Router::new()
        .nest_service("/", get_service(ServeDir::new("./")))
}

#[derive(Debug, Deserialize, Serialize)]
struct HelloParams {
    name: Option<String>,
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
        .route("/hello/:name", get(hello2_handler))
}

// eg., `/hello?name=Sagar`
async fn hello_handler(
    Query(params): Query<HelloParams>,
) -> impl IntoResponse {
    println!(
        "->> {:<12} - hello_handler - {params:?}",
        "HANDLER"
    );
    let name = params.name.as_deref().unwrap_or("World");
    format!("Hello, {}!", name).into_response()
}

// eg., `/hello/Sagar`
async fn hello2_handler(
    Path(name): Path<String>,
) -> impl IntoResponse {
    println!("->> {:<12} - hello2_handler - {name}", "HANDLER");
    // let name = params.name.as_deref().unwrap_or("World");
    format!("Hello, {}!", name).into_response()
}
