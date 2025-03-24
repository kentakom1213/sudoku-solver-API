mod field;
mod solver;
mod sudoku;

use axum::{
    routing::{get, post},
    Router,
};
use tower_service::Service;
use worker::*;

fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/solve", post(solver::solve))
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
