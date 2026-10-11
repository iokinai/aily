use std::{
    collections::HashMap,
    env,
    sync::Arc,
    time::{Duration, Instant},
};

use axum::{Router, routing::get};
use tokio::{self, net::TcpListener, sync::RwLock};
use tower_http::cors::{AllowHeaders, CorsLayer};

use crate::{
    database::SqliteDatabase,
    http::{
        AppState,
        handlers::{guess, proxy_image_handler, start_game},
    },
    session_garbage_collector::spawn_session_gc_task,
};

mod database;
mod errors;
mod game;
mod http;
mod schema;
mod session_garbage_collector;

pub fn get_new_session_lifetime() -> Instant {
    Instant::now() + Duration::from_secs(600)
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    tracing_subscriber::fmt().with_target(false).init();

    let database = SqliteDatabase::new(env::var("AILY_DATABASE").unwrap()).unwrap();
    let state = AppState {
        database: Arc::new(database),
        sessions: Arc::new(RwLock::new(HashMap::new())),
        danbooru_id: env::var("AILY_DANBOORU_ID").unwrap().parse().unwrap(),
    };

    spawn_session_gc_task(state.clone());

    let cors = CorsLayer::new()
        .allow_origin(
            env::var("AILY_FRONTEND_URL")
                .unwrap()
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers(AllowHeaders::any());

    let app = Router::new()
        //.route("/random", get(random_post))
        .route("/start", get(start_game))
        .route("/guess", get(guess))
        .route("/proxy", get(proxy_image_handler))
        .with_state(state)
        .layer(cors);

    let listener = TcpListener::bind(env::var("AILY_BACKEND_URL").unwrap())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
