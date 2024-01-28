use confy;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

use crate::providers::Provider;

const APP_NAME: &str = "kitchen-rs";

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(with = "serde_traitobject")]
    pub provider: Box<dyn Provider + Sync>,
    pub ctf: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            provider: Box::new(crate::providers::aws::AWS {}),
            ctf: "ephemeral".to_string(),
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = confy::load(APP_NAME, None).unwrap();
}
