use std::time::{Duration, Instant};

use log::info;
use tokio::time;

use crate::http::AppState;

pub fn spawn_session_gc_task(state: AppState) {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(300));

        loop {
            interval.tick().await;

            let mut sessions = state.sessions.write().await;
            let before_count = sessions.len();

            sessions.retain(|_, session| *session.expires_at() > Instant::now());

            let removed_count = before_count - sessions.len();
            if removed_count > 0 {
                info!(
                    "Cleaned up {} expired sessions. Active sessions: {}",
                    removed_count,
                    sessions.len()
                );
            }
        }
    });
}
