use anyhow::Result;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{config::Credentials, Client, Endpoint};
use aws_sdk_s3::primitives::ByteStream;
use std::str::FromStr;

use crate::config::AppConfig;

#[derive(Clone)]
pub struct Storage {
    pub client: Client,
    pub bucket: String,
}

impl Storage {
    pub async fn new(cfg: &AppConfig) -> Result<Self> {
        let region_provider = RegionProviderChain::default_provider().or_else(cfg.s3_region.clone());

        let credentials = Credentials::new(
            cfg.s3_access_key.clone(),
            cfg.s3_secret_key.clone(),
            None,
            None,
            "docops",
        );

        let mut loader = aws_config::from_env()
            .region(region_provider)
            .credentials_provider(credentials);

        let shared = loader.load().await;

        let endpoint = Endpoint::immutable(
            aws_smithy_http::endpoint::Endpoint::from_static("http://localhost")
                .with_url(cfg.s3_endpoint.parse()?)
        );

        // Build S3 config with custom endpoint (MinIO/S3 compatible)
        let s3_config = aws_sdk_s3::config::Builder::from(&shared)
            .endpoint_resolver(endpoint)
            .force_path_style(true)
            .build();

        let client = Client::from_conf(s3_config);

        Ok(Self {
            client,
            bucket: cfg.s3_bucket.clone(),
        })
    }

    pub async fn put_object(&self, key: &str, bytes: Vec<u8>, content_type: &str) -> Result<()> {
        let body = ByteStream::from(bytes);
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .body(body)
            .send()
            .await?;
        Ok(())
    }
}
