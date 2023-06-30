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
    // corsを許可
    let cors = CorsLayer::new().allow_headers(Any).allow_origin(Any);

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/solve", post(solver::solve))
        // CORSを許可
        .layer(cors);

    Ok(router.into())
}
