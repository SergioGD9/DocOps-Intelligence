use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bytes::BytesMut;
use chrono::Utc;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{db, models::{ApiMessage, Document, UploadResponse}, AppState};

pub async fn health() -> impl IntoResponse {
    Json(ApiMessage { ok: true, message: "ok".to_string() })
}

pub async fn upload(State(st): State<AppState>, mut multipart: Multipart) -> impl IntoResponse {
    // MVP: esperamos un campo file
    let mut file_name = None;
    let mut content_type = "application/octet-stream".to_string();
    let mut data = BytesMut::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        if name != "file" { continue; }

        if let Some(fname) = field.file_name() {
            file_name = Some(fname.to_string());
        }
        if let Some(ct) = field.content_type() {
            content_type = ct.to_string();
        }

        if let Ok(bytes) = field.bytes().await {
            data.extend_from_slice(&bytes);
        }
    }

    let filename = match file_name {
        Some(f) => f,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiMessage { ok: false, message: "Missing multipart field: file".to_string() }),
            )
        }
    };

    // hash sha256
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let sha256 = format!("{:x}", hasher.finalize());

    let id = Uuid::new_v4();
    let storage_key = format!("documents/{}/{}", id, filename);

    // guardar en S3/MinIO
    if let Err(e) = st.storage.put_object(&storage_key, data.to_vec(), &content_type).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiMessage { ok: false, message: format!("Storage error: {e}") }),
        );
    }

    let doc = Document {
        id,
        filename,
        content_type,
        size_bytes: data.len() as i64,
        sha256,
        storage_key,
        status: "uploaded".to_string(),
        created_at: Utc::now(),
    };

    if let Err(e) = db::insert_document(&st.db, &doc).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiMessage { ok: false, message: format!("DB error: {e}") }),
        );
    }

    // Encolar job para IA (processing pipeline)
    let mut queued = true;
    if let Err(_e) = st.queue.push_process_document(doc.id).await {
        queued = false; // no rompemos upload, pero avisamos
    }

    (StatusCode::CREATED, Json(UploadResponse { document: doc, queued }))
}

pub async fn list_documents(State(st): State<AppState>) -> impl IntoResponse {
    match db::list_documents(&st.db).await {
        Ok(docs) => (StatusCode::OK, Json(docs)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiMessage { ok: false, message: format!("DB error: {e}") }),
        ).into_response(),
    }
}

pub async fn get_document(State(st): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match db::get_document(&st.db, id).await {
        Ok(Some(doc)) => (StatusCode::OK, Json(doc)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiMessage { ok: false, message: "Not found".to_string() }),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiMessage { ok: false, message: format!("DB error: {e}") }),
        ).into_response(),
    }
}
