use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};
use uuid::Uuid;

use crate::models::Document;

pub async fn connect(pg_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(pg_url)
        .await?;
    Ok(pool)
}

pub async fn migrate(pool: &PgPool) -> Result<()> {
    // MVP: crear tabla si no existe (conceptual, sin migrator externo)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS documents (
            id UUID PRIMARY KEY,
            filename TEXT NOT NULL,
            content_type TEXT NOT NULL,
            size_bytes BIGINT NOT NULL,
            sha256 TEXT NOT NULL,
            storage_key TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        CREATE INDEX IF NOT EXISTS idx_documents_sha256 ON documents(sha256);
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_document(pool: &PgPool, doc: &Document) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO documents (id, filename, content_type, size_bytes, sha256, storage_key, status, created_at)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
        "#,
    )
    .bind(doc.id)
    .bind(&doc.filename)
    .bind(&doc.content_type)
    .bind(doc.size_bytes)
    .bind(&doc.sha256)
    .bind(&doc.storage_key)
    .bind(&doc.status)
    .bind(doc.created_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_documents(pool: &PgPool) -> Result<Vec<Document>> {
    let docs = sqlx::query_as::<_, Document>(
        r#"SELECT id, filename, content_type, size_bytes, sha256, storage_key, status, created_at
           FROM documents
           ORDER BY created_at DESC"#,
    )
    .fetch_all(pool)
    .await?;
    Ok(docs)
}

pub async fn get_document(pool: &PgPool, id: Uuid) -> Result<Option<Document>> {
    let doc = sqlx::query_as::<_, Document>(
        r#"SELECT id, filename, content_type, size_bytes, sha256, storage_key, status, created_at
           FROM documents
           WHERE id = $1"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(doc)
}

pub async fn update_status(pool: &PgPool, id: Uuid, status: &str) -> Result<()> {
    sqlx::query(r#"UPDATE documents SET status = $2 WHERE id = $1"#)
        .bind(id)
        .bind(status)
        .execute(pool)
        .await?;
    Ok(())
}
