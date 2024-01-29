pub mod aws;
use async_trait::async_trait;
use serde_traitobject::{Deserialize, Serialize};

use std::fmt::Debug;

#[async_trait]
pub trait Provider: Debug + Send + Sync {
    async fn find_optimal_region(&self) -> String;
    async fn migrate_region(&self, region: String);
}

#[async_trait]
pub trait ProviderConfig: Debug + Serialize + Deserialize + Send {
    async fn create_provider(&self) -> Box<dyn Provider>;
}
