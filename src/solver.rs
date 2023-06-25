use super::field::Field;
use super::sudoku::Sudoku;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};

const NULL: Field = Field {
    field: [
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
};

/// ## solver
pub async fn solve(Json(input): Json<Field>) -> impl IntoResponse {
    // 入力の解析
    let mut sudoku = Sudoku {
        field: input,
        answer: None,
    };

    // 解を求める
    sudoku.solve();

    // 値を返す
    if let Some(res) = sudoku.answer {
        (StatusCode::CREATED, Json(res))
    } else {
        (StatusCode::CREATED, Json(NULL))
    }
}
