use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::{database::Database, game::Session};

#[derive(Clone)]
pub struct AppState {
    pub database: Arc<dyn Database + Send + Sync>,
    pub sessions: Arc<RwLock<HashMap<String, Session>>>,
    pub danbooru_id: u64,
}
