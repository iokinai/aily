use serde::Serialize;
use trl::Trl;

use crate::http::HttpArtwork;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HttpSessionEnded {
    Yes { won: bool, next: HttpArtwork },
    No,
}

#[derive(Serialize, Trl)]
#[serde(rename_all = "snake_case")]
#[constructor]
pub struct HttpTagGuessResult {
    is_current_guess_correct: bool,
    ended: HttpSessionEnded,
}
