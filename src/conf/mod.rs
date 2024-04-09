use std::fs::File;
use std::io::BufReader;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub nostr: Nostr,
    pub postgres: Postgres,
    pub user: User,
    pub base_url: BaseUrl,
}

#[derive(Debug, Deserialize)]
pub struct BaseUrl {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Nostr {
    pub relays: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Postgres {
    pub dsn: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub name: Vec<String>,
}

pub fn load_conf(config_path: &str) -> Result<Config> {
    let file = File::open(config_path)?;
    let reader = BufReader::new(file);
    let conf = serde_yaml::from_reader(reader)?;
    Ok(conf)
}
