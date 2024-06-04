use crate::enums::errors::internal::{to_internal, InternalError};
use std::process::{Command, Output};

pub fn get_used_ips() -> Result<String, InternalError> {
    let result: Output = Command::new("wg")
        .arg("show")
        .arg("wg0")
        .arg("allowed-ips")
        .output()
        .map_err(to_internal)?;

    Ok(String::from_utf8_lossy(&result.stdout).to_string())
}
