use anyhow::{bail, Result};
use kube::Api;
use kube_custom_resources_rs::snapshot_storage_k8s_io::v1::volumesnapshotcontents::VolumeSnapshotContent;

/// Get the snapshot handle from the VolumeSnapshotContent
///
/// # Arguments
///
/// * `vsc_api` - Api object for VolumeSnapshotContent
/// * `volume_snapshot_content_name` - Name of the VolumeSnapshotContent resource
///
/// # Returns
///
/// Snapshot handle
pub async fn get_snapshot_handle(
    vsc_api: Api<VolumeSnapshotContent>,
    volume_snapshot_content_name: &str,
) -> Result<String> {
    let volume_snapshot_content = vsc_api.get(volume_snapshot_content_name).await?;

    if let Some(status) = volume_snapshot_content.status {
        Ok(status.snapshot_handle.unwrap())
    } else {
        bail!("Status of VolumeSnapshotContent is not available")
    }
}
