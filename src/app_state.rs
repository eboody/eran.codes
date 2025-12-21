use crate::{
    config::AppConfig,
    error::{Error, Result},
};

pub(crate) struct AppState {}

impl AppState {
    pub async fn init(cfg: AppConfig) -> Result<Self> {
        // here we initialize various components of the application
        // like repos

        let infra = infra::Infra::init(cfg.infra).await.map_err(Error::Infra)?;

        Ok(Self {})
    }
}
