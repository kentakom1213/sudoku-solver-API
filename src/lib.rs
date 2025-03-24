mod field;
mod solver;
mod sudoku;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_service::Service;
use worker::*;

fn router() -> Router {
    // corsを許可
    let cors = CorsLayer::new().allow_headers(Any).allow_origin(Any);

    Router::new()
        .route("/", get(root))
        .route("/solve", post(solver::solve))
        .layer(cors)
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

pub async fn root() -> &'static str {
    "Sudoku Solver!"
}
