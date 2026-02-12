mod config;
mod db;
mod models;
mod routes;
mod storage;
mod queue;

use axum::{routing::{get, post}, Router};
use sqlx::PgPool;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

use crate::config::AppConfig;
use crate::storage::Storage;
use crate::queue::Queue;

#[derive(Clone)]
pub struct AppState {
    pub cfg: AppConfig,
    pub db: PgPool,
    pub storage: Storage,
    pub queue: Queue,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cfg = AppConfig::from_env();

    let db = db::connect(&cfg.postgres_url).await?;
    db::migrate(&db).await?;

    let storage = Storage::new(&cfg).await?;
    let queue = Queue::new(&cfg.redis_url)?;

    let state = AppState { cfg: cfg.clone(), db, storage, queue };

    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/upload", post(routes::upload))
        .route("/documents", get(routes::list_documents))
        .route("/documents/:id", get(routes::get_document))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = cfg.server_addr.clone();
    tracing::info!("DocOps API running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
