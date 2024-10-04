use super::region::get_region_config;
use anyhow::Result;
use aws_sdk_ec2::Client as EbsClient;

/// Create an EBS client
///
/// # Arguments
///
/// * `region` - AWS region
///
/// # Returns
///
/// EBS client
pub async fn create_ebs_client(region: Option<String>) -> Result<EbsClient> {
    let region_config = get_region_config(region).await;
    let ebs_client = EbsClient::new(&region_config);
    Ok(ebs_client)
}

/// Get the progress of an EBS snapshot
///
/// # Arguments
///
/// * `ebs_client` - EBS client
/// * `snapshot_id` - Snapshot ID
///
/// # Returns
///
/// Progress of the snapshot
pub async fn get_ebs_snapshot_progress(
    ebs_client: EbsClient,
    snapshot_id: String,
) -> Result<String> {
    let resp = ebs_client
        .describe_snapshots()
        .snapshot_ids(snapshot_id)
        .send()
        .await?;

    let snapshot = resp.snapshots.unwrap().pop().unwrap();
    let progress = snapshot.progress.unwrap();
    Ok(progress)
}
