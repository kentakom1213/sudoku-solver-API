use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use sudoku::solver;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/solve", post(solver::solve));

    // アプリケーションの定義
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/solve", post(solver::solve));

    // 実行
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // デバッグログ
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
