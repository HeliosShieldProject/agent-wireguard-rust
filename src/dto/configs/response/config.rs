use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub private_key: String,
    pub user_ip: String,
}

impl Config {
    pub fn new(private_key: String, user_ip: String) -> Self {
        Self {
            private_key,
            user_ip,
        }
    }
}
