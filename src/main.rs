use axum::{
    routing::{get, post},
    Router,
};
use sudoku::solver;
use tower_http::cors::{Any, CorsLayer};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    // CORSを許可
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/solve", post(solver::solve))
        .layer(cors);

    Ok(router.into())
}
