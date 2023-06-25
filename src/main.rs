use axum::{
    routing::{get, post},
    Router,
};
use sudoku::solver;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/solve", post(solver::solve));

    Ok(router.into())
}
