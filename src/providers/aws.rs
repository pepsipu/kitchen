use crate::providers::{Provider, ProviderConfig};
use async_trait::async_trait;
use aws_config::{provider_config::ProviderConfig as AWSProviderConfig, BehaviorVersion};
use aws_sdk_ec2::Client as Ec2Client;
use serde_derive::{Deserialize, Serialize};
use tokio::task::JoinSet;

#[derive(Debug)]
pub struct AWS {
    ec2_client: Ec2Client,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AWSConfig {
    pub region: Option<String>,
}

#[async_trait]
impl ProviderConfig for AWSConfig {
    async fn create_provider(&self) -> Box<dyn Provider> {
        Box::new(AWS::new().await)
    }
}

impl Default for AWSConfig {
    fn default() -> Self {
        AWSConfig { region: None }
    }
}

impl AWS {
    pub async fn new() -> AWS {
        let config = aws_config::load_defaults(BehaviorVersion::v2023_11_09()).await;
        let ec2_client = Ec2Client::new(&config);

        AWS { ec2_client }
    }

    // Function to measure latency to a region
    async fn measure_latency(&self, region: &str) -> f64 {
        // Placeholder: Replace with actual method to measure latency
        // For example, sending a request to a known endpoint in the region and measuring response time
        0.0 // Dummy value
    }
}

#[async_trait]
impl Provider for AWS {
    async fn find_optimal_region(&self) -> String {
        // Fetch the list of regions from AWS
        let regions = self
            .ec2_client
            .describe_regions()
            .send()
            .await
            .unwrap()
            .regions
            .unwrap();

        for region_info in regions {
            let region_name = region_info.region_name.as_ref().unwrap().clone();
            let latency = self.measure_latency(&region_name).await;
            println!("Latency to {}: {}", region_name, latency);
        }

        "meow".to_string()
    }

    async fn migrate_region(&self, region: String) {
        // Implementation for region migration
    }
}
