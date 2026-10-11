use std::{
    collections::HashMap,
    env,
    sync::Arc,
    time::{Duration, Instant},
};

use axum::{Router, routing::get};
use axum_extra::extract::cookie::Cookie;
use reqwest::header;
use tokio::{self, net::TcpListener, sync::RwLock};
use tower_http::cors::CorsLayer;

use crate::{
    database::SqliteDatabase,
    http::{
        AppState,
        handlers::{current_session, current_session_post, guess, proxy_image_handler, start_game},
    },
    session_garbage_collector::spawn_session_gc_task,
};

mod database;
mod errors;
mod game;
mod http;
mod schema;
mod session_garbage_collector;

pub fn get_new_session_lifetime() -> Duration {
    Duration::from_secs(7200)
}

pub fn get_new_session_exptime() -> Instant {
    Instant::now() + get_new_session_lifetime()
}

pub fn make_new_session_cookie<'a>(id: String) -> Cookie<'a> {
    Cookie::build(("session_id", id))
        .max_age(time::Duration::seconds(
            get_new_session_lifetime().as_secs() as i64,
        ))
        .http_only(true)
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .path("/")
        .secure(!cfg!(debug_assertions))
        .build()
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
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true);

    let app = Router::new()
        //.route("/random", get(random_post))
        .route("/start", get(start_game))
        .route("/guess", get(guess))
        .route("/proxy", get(proxy_image_handler))
        .route("/current", get(current_session_post))
        .route("/session", get(current_session))
        .with_state(state)
        .layer(cors);

    let listener = TcpListener::bind(env::var("AILY_BACKEND_URL").unwrap())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
