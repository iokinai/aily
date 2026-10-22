use trl::Trl;

#[derive(Trl, Clone, Debug)]
#[getters]
#[constructor]
pub struct GameAilySession {
    artwork_id: i32,
    guessed_correctly: bool,
}
