use anyhow::Result;
use redis::AsyncCommands;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct QueueJob {
    pub job_type: String,     // "process_document"
    pub document_id: Uuid,
}

#[derive(Clone)]
pub struct Queue {
    client: redis::Client,
    list_name: String,
}

impl Queue {
    pub fn new(redis_url: &str) -> Result<Self> {
        let client = redis::Client::open(redis_url)?;
        Ok(Self { client, list_name: "docops:jobs".to_string() })
    }

    pub async fn push_process_document(&self, document_id: Uuid) -> Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let job = QueueJob {
            job_type: "process_document".to_string(),
            document_id,
        };
        let payload = serde_json::to_string(&job)?;
        let _: i64 = conn.lpush(&self.list_name, payload).await?;
        Ok(())
    }
}
