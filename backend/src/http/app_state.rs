use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::{database::Database, game::GameSession};

#[derive(Clone)]
pub struct AppState {
    pub database: Arc<dyn Database + Send + Sync>,
    pub sessions: Arc<RwLock<HashMap<String, GameSession>>>,
    pub danbooru_id: u64,
}
