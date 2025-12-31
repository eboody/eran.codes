mod config;
mod error;

use std::sync::Arc;

use app::user;
use error::{Error, Result};
use infra::user::Repository as UserRepo;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::Config::load()?;

    let addr = format!("{}:{}", &cfg.http.host, cfg.http.port);

    let infra = infra::Infra::init(&cfg.infra).await.map_err(Error::Infra)?;

    let user_repo = Arc::new(UserRepo::new(infra.db.clone()));
    let user_service = user::Service::new(user_repo);

    let http_state = http::State::new(user_service);

    let app = http::router(http_state);

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("listening on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
