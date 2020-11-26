pub mod game;

use wasm_bindgen::prelude::*;
use game::Game;

#[wasm_bindgen]
pub fn rows() -> u32 {
    game::ROWS as u32
}

#[wasm_bindgen]
pub fn cols() -> u32 {
    game::COLS as u32
}

#[wasm_bindgen]
pub fn wins() -> u32 {
    game::WIN as u32
}

#[wasm_bindgen]
pub fn calculate_move(board: JsValue, moves: u32) -> u32 {
    let mut game = Game::from(board);
    let scores = game.minimax(moves);
    let mut best_col = 0;
    let mut best_score = i8::MIN;
    for i in 0..scores.len() {
        if scores[i] > best_score {
            best_score = scores[i];
            best_col = i;
        }
    }
    best_col as u32
}

#[wasm_bindgen]
pub fn calculate_scores(board: JsValue, moves: u32) -> JsValue {
    let mut game = Game::from(board);
    let scores = game.minimax(moves);
    JsValue::from_serde(&scores).unwrap()
}