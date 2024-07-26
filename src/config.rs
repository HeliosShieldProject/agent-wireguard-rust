use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub agent_host: String,
    pub agent_port: u16,
}

pub static ENV: Lazy<Config> = Lazy::new(|| {
    dotenv().ok();
    Config {
        agent_host: env::var("AGENT_HOST").expect("AGENT_HOST must be set"),
        agent_port: env::var("AGENT_PORT")
            .expect("AGENT_PORT must be set")
            .parse()
            .expect("AGENT_PORT must be a number"),
    }
});
