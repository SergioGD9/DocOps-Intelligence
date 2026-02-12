use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Document {
    pub id: Uuid,
    pub filename: String,
    pub content_type: String,
    pub size_bytes: i64,
    pub sha256: String,
    pub storage_key: String,
    pub status: String, // uploaded | processing | indexed | failed
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ApiMessage {
    pub ok: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub document: Document,
    pub queued: bool,
}
