use crate::{
    dto::response::success::Response,
    enums::errors::external::{to_external, ExternalError},
    services::config_service,
};
use axum::http::StatusCode;
use tracing::info;

pub async fn add_config() -> Result<Response<String>, ExternalError> {
    let private_key = config_service::add_config().await.map_err(to_external)?;

    info!("Config added: {}{}", &private_key[..10], &"*".repeat(private_key.len() - 10));
    Ok(Response::new(StatusCode::OK, "Config added").with_data(private_key))
}
