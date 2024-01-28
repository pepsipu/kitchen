// serilize
use serde_derive::{Deserialize, Serialize};

use crate::providers::Provider;

#[derive(Serialize, Deserialize)]
pub struct AWS {}

impl Provider for AWS {}
