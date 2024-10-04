use crate::k8s_ops::{
    pvc::{
        persistent_volume_claims::{check_if_pvc_exists, get_pvcs_available, KubePvcApi},
        persistent_volume_claims_operator::PVCOperator,
        persistent_volume_claims_payload::PVCOperatorPayload,
    },
    vs::volume_snapshots_operator::VolumeSnapshotOperator,
    vsc::{
        volume_snapshot_contents::get_snapshot_handle,
        volume_snapshot_contents_operator::VolumeSnapshotContentOperator,
    },
};
use anyhow::{bail, Result};
use kube::{api::PostParams, Api, Client};
use kube_custom_resources_rs::snapshot_storage_k8s_io::v1::{
    volumesnapshotcontents::VolumeSnapshotContent,
    volumesnapshots::{VolumeSnapshot, VolumeSnapshotStatus},
};
use tracing::info;

use super::restore_payload::RestorePayload;

/// A struct for restoring a PVC from a VolumeSnapshot
pub struct RestoreOperator;

impl RestoreOperator {
    /// Restores one or more PVCs from a VolumeSnapshot to a specific namespace
    pub async fn restore(restore_payload: RestorePayload) -> Result<()> {
        // Create a Kubernetes client
        let k8s_client = Client::try_default().await?;

        // Define the VolumeSnapshot, VolumeSnapshotContent and PersistentVolumeClaim APIs
        let restore_k8s_apis_struct = RestoreKubernetesApisStruct {
            source_vs_api: Api::namespaced(k8s_client.clone(), restore_payload.source_ns()),
            source_pvcs_api: KubePvcApi {
                api: Api::namespaced(k8s_client.clone(), restore_payload.source_ns()),
            },
            target_vs_api: Api::namespaced(k8s_client.clone(), restore_payload.target_ns()),
            target_pvcs_api: KubePvcApi {
                api: Api::namespaced(k8s_client.clone(), restore_payload.target_ns()),
            },
            vsc_api: Api::all(k8s_client.clone()),
        };

        // Check if we will restore all PVCs in the namespace
        let pvcs = if restore_payload.include_all_pvcs() {
            get_pvcs_available(&restore_k8s_apis_struct.source_pvcs_api).await?
        } else {
            vec![restore_payload
                .pvc_name()
                .unwrap_or_else(|| panic!("PVC name is required when include_all_pvcs is false"))
                .to_string()]
        };

        // We will iterate over the PVCs vector and restore each PVC

        for pvc in pvcs {
            info!("Restoring PVC: {}", pvc);
            let volume_snapshot_name = format!("{}-{}", restore_payload.vs_name_prefix(), pvc);
            let volume_snapshot_content_name =
                format!("{}-{}", restore_payload.vsc_name_prefix(), pvc);

            // Check if the PVC exists in the target namespace, it should not exist
            check_if_pvc_exists(&restore_k8s_apis_struct.target_pvcs_api, &pvc, false).await?;

            let status: VolumeSnapshotStatus = match restore_k8s_apis_struct
                .source_vs_api
                .get(&volume_snapshot_name)
                .await
            {
                Ok(snapshot) => snapshot.status.unwrap(),
                Err(e) => {
                    bail!("Failed to get VolumeSnapshot status: {}", e)
                }
            };

            let bound_vsc_name = status.bound_volume_snapshot_content_name.unwrap();
            let restore_size = status.restore_size.unwrap();

            let snapshot_handle =
                get_snapshot_handle(restore_k8s_apis_struct.vsc_api.clone(), &bound_vsc_name)
                    .await?;

            let vsc_operator = VolumeSnapshotContentOperator::new(
                volume_snapshot_content_name.clone(),
                restore_payload.target_ns().to_string(),
                volume_snapshot_name.clone(),
                Some(restore_payload.volume_snapshot_class().to_string()),
                Some(snapshot_handle.clone()),
                *restore_payload.vsc_retain_policy(),
            );

            let snapshot_content = vsc_operator.construct_volume_snapshot_content_resource();

            let pp = PostParams::default();
            match restore_k8s_apis_struct
                .vsc_api
                .create(&pp, &snapshot_content)
                .await
            {
                Ok(snapshot_content) => {
                    info!(
                        "{}",
                        format!(
                            "Created VolumeSnapshotContent: {} on namespace: {}",
                            snapshot_content.metadata.name.clone().unwrap(),
                            restore_payload.target_ns()
                        )
                    )
                }
                Err(e) => panic!("Failed to create VolumeSnapshotContent: {}", e),
            }

            let vs_operator = VolumeSnapshotOperator::new(
                volume_snapshot_name.clone(),
                restore_payload.target_ns().to_string(),
                restore_payload.volume_snapshot_class().to_string(),
                None,
                Some(volume_snapshot_content_name),
            );

            let target_volume_snapshot = vs_operator.construct_volume_snapshot_resource(
                Some(snapshot_handle.to_string()),
                Some(restore_size.to_string()),
                *restore_payload.vsc_retain_policy(),
            );

            info!("Creating VolumeSnapshot in the target namespace...");
            let pp = PostParams::default();
            match restore_k8s_apis_struct
                .target_vs_api
                .create(&pp, &target_volume_snapshot)
                .await
            {
                Ok(target_volume_snapshot) => {
                    info!(
                        "{}",
                        format!(
                            "Created VolumeSnapshot: {} on namespace: {}",
                            target_volume_snapshot.metadata.name.clone().unwrap(),
                            restore_payload.target_ns()
                        )
                    )
                }
                Err(e) => panic!("Failed to create VolumeSnapshot: {}", e),
            }

            // Restore the PVC for each pvc available
            let pvc_payload = PVCOperatorPayload::new(
                pvc,
                restore_payload.target_ns(),
                Some(restore_payload.storage_class_name().to_string()),
                None,
                volume_snapshot_name,
                restore_size,
            );

            let pvc_operator = PVCOperator::new(pvc_payload);
            let pvc = pvc_operator.construct_persistent_volume_claim_resource();

            info!("Restoring PVC...");
            let pp = PostParams::default();
            match restore_k8s_apis_struct
                .target_pvcs_api
                .api
                .create(&pp, &pvc)
                .await
            {
                Ok(pvc) => info!(
                    "{}",
                    format!(
                        "Restored PVC: {} on namespace: {}",
                        pvc.metadata.name.clone().unwrap(),
                        restore_payload.target_ns()
                    )
                ),
                Err(e) => panic!("Failed to restore PVC: {}", e),
            }
        }

        Ok(())
    }
}

/// A struct for holding the Kubernetes APIs for the restore operation
struct RestoreKubernetesApisStruct {
    source_vs_api: Api<VolumeSnapshot>,
    source_pvcs_api: KubePvcApi,
    target_vs_api: Api<VolumeSnapshot>,
    target_pvcs_api: KubePvcApi,
    vsc_api: Api<VolumeSnapshotContent>,
}
