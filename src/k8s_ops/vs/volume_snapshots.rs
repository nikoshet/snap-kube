use crate::aws_ops::ebs::get_ebs_snapshot_progress;
use crate::k8s_ops::vsc::volume_snapshot_contents::get_snapshot_handle;
use anyhow::Result;
use aws_sdk_ec2::Client as EbsClient;
use kube_custom_resources_rs::snapshot_storage_k8s_io::v1::volumesnapshotcontents::VolumeSnapshotContent;
use kube_custom_resources_rs::snapshot_storage_k8s_io::v1::volumesnapshots::{
    VolumeSnapshot, VolumeSnapshotStatus,
};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};

/// Wait untill the VolumeSnapshot is ready
///
/// # Arguments
///
/// * `vs_api` - Api object for VolumeSnapshot
/// * `vsc_api` - Api object for VolumeSnapshotContent
/// * `ebs_client` - EBS Client object
/// * `volume_snapshot_name` - Name of the VolumeSnapshot resource
///
/// # Returns
///
/// VolumeSnapshotStatus
pub async fn wait_untill_snapshot_is_ready(
    vs_api: &kube::Api<VolumeSnapshot>,
    vsc_api: &kube::Api<VolumeSnapshotContent>,
    ebs_client: &EbsClient,
    volume_snapshot_name: &str,
) -> Result<VolumeSnapshotStatus> {
    loop {
        let snapshot = vs_api.get(volume_snapshot_name).await?;
        if let Some(status) = snapshot.status {
            if status.ready_to_use.unwrap_or(false) {
                info!("Snapshot is ready: {:?}", status);
                return Ok(status);
            }
            info!("Waiting for VolumeSnapshot to be ready...");

            let vsc_name = status.bound_volume_snapshot_content_name.unwrap();
            match get_snapshot_handle(vsc_api.clone(), &vsc_name).await {
                Ok(snapshot_handle) => {
                    let progress =
                        get_ebs_snapshot_progress(ebs_client.clone(), snapshot_handle.clone())
                            .await?;
                    info!(
                        "{}",
                        format!(
                            "Progress for EBS snapshot {} regarding VS {} is: {}",
                            snapshot_handle, volume_snapshot_name, progress
                        )
                    );
                }
                Err(e) => {
                    warn!("Failed to get snapshot handle: {}", e);
                }
            }
            sleep(Duration::from_secs(5)).await;
        }
    }
}
