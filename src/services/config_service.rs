use crate::{dto::configs::response::Config, enums::errors::internal::InternalError, scripts};
use std::net::Ipv4Addr;
use tracing::error;

pub async fn add_config() -> Result<Config, InternalError> {
    let ip: Ipv4Addr = get_available_ip().await?;
    let private_key: String = scripts::add_config(ip).map_err(|e| {
        error!("Failed to add config: {}", e);
        e
    })?;

    Ok(Config::new(private_key, ip.to_string()))
}

pub async fn get_used_ips() -> Result<Vec<Ipv4Addr>, InternalError> {
    let used_ips_raw = scripts::get_used_ips().map_err(|e| {
        error!("Failed to get used IPs: {}", e);
        e
    })?;
    let used_ips: Vec<Ipv4Addr> = used_ips_raw
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            parts.next().unwrap();
            let ip = parts.next().unwrap().trim_end_matches("/32");
            ip.parse().unwrap()
        })
        .collect();

    Ok(used_ips)
}

pub async fn get_available_ip() -> Result<Ipv4Addr, InternalError> {
    let used_ips = get_used_ips().await?;
    let mut ip = Ipv4Addr::new(10, 0, 0, 2);

    while used_ips.contains(&ip) {
        let octets = ip.octets();
        if octets[3] == 255 {
            ip = Ipv4Addr::new(octets[0], octets[1], octets[2] + 1, 1);
        } else {
            ip = Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3] + 1);
        }
    }

    Ok(ip)
}
