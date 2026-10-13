use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use axum::{
    Router,
    routing::{get, post},
};
use axum_extra::extract::cookie::Cookie;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use log::{error, info};
use reqwest::header;
use tokio::{self, net::TcpListener, sync::RwLock};
use tower_http::cors::CorsLayer;

use crate::{
    database::SqliteDatabase,
    environment::Environment,
    http::{
        AppState,
        handlers::{get_current_session, make_guess, proxy_image_handler, start_new_session},
    },
    session_garbage_collector::spawn_session_gc_task,
};

mod database;
mod environment;
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

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

fn run_migrations(conn: &mut SqliteConnection) {
    conn.run_pending_migrations(MIGRATIONS)
        .inspect_err(|e| error!("Could not run migrations: {}", e))
        .unwrap();
}

#[tokio::main]
async fn main() {
    let env = Environment::parse().unwrap();
    tracing_subscriber::fmt().with_target(false).init();

    info!("Running database migrations");

    run_migrations(&mut SqliteConnection::establish(env.database_connection()).unwrap());

    info!("Migrations ran successfully");

    let database = SqliteDatabase::new(env.database_connection().clone()).unwrap();

    info!("Database connection estabilished");

    let client = reqwest::Client::builder()
        .user_agent(format!("Aily /1.0 (user #{})", env.danbooru_id()))
        .build()
        .unwrap();

    let state = AppState {
        database: Arc::new(database),
        sessions: Arc::new(RwLock::new(HashMap::new())),
        http_request_client: Arc::new(client),
    };

    spawn_session_gc_task(state.clone());

    info!("Sessions garbage collector started");

    let cors = CorsLayer::new()
        .allow_origin(
            env.frontend_url()
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true);

    let api_routes = Router::new()
        .route("/session", get(get_current_session).post(start_new_session))
        .route("/proxy", get(proxy_image_handler));

    let aily_routes = Router::new().route("/", post(make_guess));

    let app = Router::new()
        .nest(
            "/api",
            Router::new().merge(api_routes).nest("/aily", aily_routes),
        )
        .with_state(state)
        .layer(cors);

    let backend_url = env.backend_url();

    let listener = TcpListener::bind(backend_url.clone()).await.unwrap();

    info!("TCP listener bound to {}", backend_url);

    axum::serve(listener, app).await.unwrap();
}
