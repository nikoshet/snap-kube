use aws_config::meta::region::RegionProviderChain;
use aws_config::retry::RetryConfig;
use aws_config::{BehaviorVersion, Region, SdkConfig};

///
/// Get the AWS region configuration
///
/// # Arguments
///
/// * `region` - AWS region
///
/// # Returns
///
/// AWS configuration
pub async fn get_region_config(region: Option<String>) -> SdkConfig {
    let main_region = region.unwrap_or("eu-west-1".to_string());

    let region_provider = RegionProviderChain::first_try(Region::new(main_region))
        .or_default_provider()
        .or_else(Region::new("eu-west-1"));

    aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .retry_config(RetryConfig::adaptive())
        .load()
        .await
}
