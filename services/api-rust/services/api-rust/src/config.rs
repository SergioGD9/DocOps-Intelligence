use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub server_addr: String,
    pub postgres_url: String,

    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub s3_region: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,

    pub redis_url: String,
    pub ai_service_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            server_addr: env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
            postgres_url: env::var("POSTGRES_URL").unwrap_or_else(|_| "postgres://docops:docops@localhost:5432/docops".to_string()),

            s3_endpoint: env::var("S3_ENDPOINT").unwrap_or_else(|_| "http://localhost:9000".to_string()),
            s3_bucket: env::var("S3_BUCKET").unwrap_or_else(|_| "docops".to_string()),
            s3_region: env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
            s3_access_key: env::var("S3_ACCESS_KEY").unwrap_or_else(|_| "minio".to_string()),
            s3_secret_key: env::var("S3_SECRET_KEY").unwrap_or_else(|_| "minio123456".to_string()),

            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            ai_service_url: env::var("AI_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string()),
        }
    }
}
