use serde_derive::{Deserialize, Serialize};

use crate::providers::{Provider, ProviderConfig};

use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use tokio::sync::OnceCell;

pub const APP_NAME: &str = "kitchen-rs";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(with = "serde_traitobject")]
    pub provider: Box<dyn ProviderConfig + Sync>,
    pub ctf: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            provider: Box::new(crate::providers::aws::AWSConfig::default()),
            ctf: "ephemeral".to_string(),
        }
    }
}

// Define the Config and Provider traits or structs here
// ...

lazy_static! {
    static ref CONFIG: OnceCell<Config> = OnceCell::new();
    static ref PROVIDER: OnceCell<Box<dyn Provider>> = OnceCell::new();
}

pub fn set_cfg(cfg: Config) {
    CONFIG.set(cfg).unwrap();
}

pub fn set_provider(provider: Box<dyn Provider>) {
    PROVIDER.set(provider).unwrap();
}

pub fn get_cfg() -> &'static Config {
    CONFIG.get().unwrap()
}

pub fn get_provider() -> &'static Box<dyn Provider> {
    PROVIDER.get().unwrap()
}
