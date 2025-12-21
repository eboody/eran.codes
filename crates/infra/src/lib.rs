pub mod config;
pub mod error;
pub mod repo;

use sqlx::PgPool;

use crate::error::Result;

// our infra layer holds shared resources like DB pools, HTTP clients, etc.
pub struct Infra {
    pub db: PgPool,
    pub http: reqwest::Client,
}

impl Infra {
    pub async fn init(cfg: config::InfraConfig) -> Result<Self> {
        let pool = PgPool::connect(&cfg.database_url)
            .await
            .map_err(error::Error::Pgsql)?;

        pool.acquire().await.map_err(error::Error::Pgsql)?;

        sqlx::migrate!().run(&pool).await?;

        let http_client = reqwest::Client::builder()
            .build()
            .map_err(error::Error::HttpClient)?;

        Ok(Self {
            db: pool,
            http: http_client,
        })
    }
}
