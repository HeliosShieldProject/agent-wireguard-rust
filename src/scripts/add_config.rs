use crate::enums::errors::internal::{to_internal, InternalError};
use std::{
    fs::File,
    io::Write,
    net::Ipv4Addr,
    process::{Command, Stdio},
};

pub fn add_config(ip: Ipv4Addr) -> Result<String, InternalError> {
    let private_key_output = Command::new("wg")
        .arg("genkey")
        .output()
        .map_err(to_internal)?;

    if !private_key_output.status.success() {
        return Err(InternalError::InternalError);
    }

    let private_key = String::from_utf8(private_key_output.stdout).map_err(to_internal)?;

    let public_key_output = Command::new("wg")
        .arg("pubkey")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child
                .stdin
                .as_mut()
                .unwrap()
                .write_all(private_key.as_bytes())?;
            child.wait_with_output()
        })
        .map_err(to_internal)?;

    if !public_key_output.status.success() {
        return Err(InternalError::InternalError);
    }

    let public_key = String::from_utf8(public_key_output.stdout).map_err(to_internal)?;

    Command::new("wg")
        .arg("set")
        .arg("wg0")
        .arg("peer")
        .arg(public_key.trim())
        .arg("allowed-ips")
        .arg(format!("{}/32", ip))
        .spawn()
        .map_err(to_internal)?;

    Command::new("ip")
        .arg("-4")
        .arg("route")
        .arg("add")
        .arg(format!("{}/32", ip))
        .arg("dev")
        .arg("wg0")
        .spawn()
        .map_err(to_internal)?;

    let mut wg0_conf: File = File::options()
        .append(true)
        .open("/etc/wireguard/wg0.conf")
        .map_err(to_internal)?;

    writeln!(wg0_conf, "[Peer]").map_err(to_internal)?;
    writeln!(wg0_conf, "PublicKey = {}", public_key.trim()).map_err(to_internal)?;
    writeln!(wg0_conf, "AllowedIPs = {}/32", ip).map_err(to_internal)?;

    Ok(private_key.trim().to_string())
}
