use crate::enums::errors::internal::{to_internal, InternalError};
use std::{
    env,
    fs::{self, File},
    io::Write,
    net::Ipv4Addr,
    process::Command,
};

pub fn add_config(ip: Ipv4Addr) -> Result<String, InternalError> {
    let temp_dir: &str = "temp";
    fs::create_dir_all(temp_dir).map_err(to_internal)?;
    env::set_current_dir(temp_dir).map_err(to_internal)?;

    Command::new("wg")
        .arg("genkey")
        .arg("|")
        .arg("tee")
        .arg("privatekey")
        .arg("|")
        .arg("wg")
        .arg("pubkey")
        .output()
        .map_err(to_internal)?;

    let private_key: String = fs::read_to_string("privatekey").map_err(to_internal)?;
    let public_key: String = fs::read_to_string("publickey").map_err(to_internal)?;

    Command::new("wg")
        .arg("set")
        .arg("wg0")
        .arg("peer")
        .arg(public_key.trim())
        .arg("allowed-ips")
        .arg(format!("{}/32", ip))
        .output()
        .map_err(to_internal)?;

    Command::new("ip")
        .arg("-4")
        .arg("route")
        .arg("add")
        .arg(format!("{}/32", ip))
        .arg("dev")
        .arg("wg0")
        .output()
        .map_err(to_internal)?;

    let mut wg0_conf: File = File::options()
        .append(true)
        .open("/etc/wireguard/wg0.conf")
        .map_err(to_internal)?;

    writeln!(wg0_conf, "[Peer]").map_err(to_internal)?;
    writeln!(wg0_conf, "PublicKey = {}", public_key.trim()).map_err(to_internal)?;
    writeln!(wg0_conf, "AllowedIPs = {}/32", ip).map_err(to_internal)?;

    fs::remove_dir_all(temp_dir).map_err(to_internal)?;
    env::set_current_dir("..").map_err(to_internal)?;

    Ok(private_key.trim().to_string())
}
