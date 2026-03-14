pub mod auth;
pub mod chat;
pub mod config;
mod error;
pub use error::{Error, Result};
pub mod repo;
use snafu::ResultExt;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

// our infra layer holds shared resources like DB pools, HTTP clients, etc.
pub struct Infra {
    pub db: PgPool,
    pub http: reqwest::Client,
}

impl Infra {
    #[tracing::instrument(skip(cfg))]
    pub async fn init(cfg: &config::Infra) -> Result<Self> {
        let database_url = cfg.db.url.to_string();
        let pool = PgPoolOptions::new()
            .max_connections(cfg.db.max_connections)
            .connect(cfg.db.url.as_ref())
            .await
            .context(error::ConnectDbSnafu {
                database_url: database_url.clone(),
            })?;

        pool.acquire()
            .await
            .context(error::CheckDbConnectionSnafu { database_url })?;

        tracing::info!("running database migrations");
        sqlx::migrate!()
            .run(&pool)
            .await
            .context(error::RunMigrationsSnafu)?;

        let http_client = reqwest::Client::builder()
            .build()
            .context(error::BuildHttpClientSnafu)?;

        Ok(Self {
            db: pool,
            http: http_client,
        })
    }
}
