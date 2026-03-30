use aws_config::{BehaviorVersion, Region, SdkConfig};

pub async fn create_sdk_config(region: &str) -> SdkConfig {
    aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(region.to_string()))
        .load()
        .await
}
