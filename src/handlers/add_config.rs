use crate::{
    dto::{configs::response::Config, response::success::Response},
    enums::errors::external::{to_external, ExternalError},
    services::config_service,
};
use axum::http::StatusCode;
use tracing::info;

pub async fn add_config() -> Result<Response<Config>, ExternalError> {
    let config = config_service::add_config().await.map_err(to_external)?;

    info!(
        "Config added: {}{}",
        &config.private_key[..10],
        &"*".repeat(config.private_key.len() - 10)
    );
    Ok(Response::new(StatusCode::OK, "Config added").with_data(config))
}
