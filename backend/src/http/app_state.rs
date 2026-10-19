use std::{collections::HashMap, sync::Arc};

use reqwest::Client;
use tokio::sync::RwLock;

use crate::game::{
    GameSession, aily::AilyGameController, global::GlobalGameController, tags::TagsGameController,
};

#[derive(Clone)]
pub struct AppState {
    pub sessions: Arc<RwLock<HashMap<String, GameSession>>>,
    pub http_request_client: Arc<Client>,
    pub aily_game_controller: AilyGameController,
    pub global_game_controller: GlobalGameController,
    pub tags_game_controller: TagsGameController,
}
