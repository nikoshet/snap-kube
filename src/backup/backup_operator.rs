use super::backup_payload::BackupPayload;
use crate::{
    aws_ops::ebs::create_ebs_client,
    k8s_ops::{
        pvc::persistent_volume_claims::{check_if_pvc_exists, get_pvcs_available, KubePvcApi},
        vs::{
            volume_snapshots::wait_untill_snapshot_is_ready,
            volume_snapshots_operator::VolumeSnapshotOperator,
        },
        vsc::retain_policy::VSCRetainPolicy,
    },
};
use anyhow::{bail, Result};
use kube::{api::PostParams, Api, Client};
use kube_custom_resources_rs::snapshot_storage_k8s_io::v1::{
    volumesnapshotcontents::VolumeSnapshotContent,
    volumesnapshots::{VolumeSnapshot, VolumeSnapshotStatus},
};
use tracing::info;

/// A struct for backing up a PVC to a VolumeSnapshot
pub struct BackupOperator;

impl BackupOperator {
    /// Takes a backup of one or more PVCs from a specific namespace to a VolumeSnapshot/VolumeSnapshotContent
    pub async fn backup(backup_payload: BackupPayload) -> Result<()> {
        // Create a Kubernetes client
        let k8s_client = Client::try_default().await?;

        // Create an AWS EBS client
        let ebs_client = create_ebs_client(Some(backup_payload.region().to_string()))
            .await
            .unwrap();

        // Define the VolumeSnapshot and VolumeSnapshotContent APIs
        let restore_k8s_apis_struct = BackupKubernetesApisStruct {
            source_vs_api: Api::namespaced(k8s_client.clone(), backup_payload.source_ns()),
            source_pvcs_api: KubePvcApi {
                api: Api::namespaced(k8s_client.clone(), backup_payload.source_ns()),
            },
            vsc_api: Api::all(k8s_client.clone()),
        };

        // Check if we will backup all PVCs in the namespace
        let pvcs = if backup_payload.include_all_pvcs() {
            get_pvcs_available(&restore_k8s_apis_struct.source_pvcs_api).await?
        } else {
            vec![backup_payload
                .pvc_name()
                .unwrap_or_else(|| panic!("PVC name is required when include_all_pvcs is false"))
                .to_string()]
        };

        // We will iterate over the PVCs vector and backup each PVC
        for pvc in pvcs {
            info!("Backing up PVC: {}", pvc);
            let volume_snapshot_name = format!("{}-{}", backup_payload.vs_name_prefix(), pvc);

            // Check if the PVC exists, it should exist
            check_if_pvc_exists(&restore_k8s_apis_struct.source_pvcs_api, &pvc, true).await?;

            let vs_operator = VolumeSnapshotOperator::new(
                volume_snapshot_name.to_string(),
                backup_payload.source_ns.to_string(),
                backup_payload.volume_snapshot_class().to_string(),
                Some(pvc),
                None,
            );

            let volume_snapshot =
                vs_operator.construct_volume_snapshot_resource(None, None, VSCRetainPolicy::Delete);

            let pp = PostParams::default();
            let status: VolumeSnapshotStatus = match restore_k8s_apis_struct
                .source_vs_api
                .create(&pp, &volume_snapshot)
                .await
            {
                Ok(snapshot) => {
                    info!(
                        "{}",
                        format!(
                            "Created VolumeSnapshot: {} on namespace: {}",
                            snapshot.metadata.name.clone().unwrap(),
                            backup_payload.source_ns()
                        )
                    );
                    wait_untill_snapshot_is_ready(
                        &restore_k8s_apis_struct.source_vs_api,
                        &restore_k8s_apis_struct.vsc_api,
                        &ebs_client,
                        &volume_snapshot_name,
                    )
                    .await?
                }
                Err(e) => {
                    bail!("Failed to create VolumeSnapshot: {}", e);
                }
            };

            let bound_vsc_name = status.bound_volume_snapshot_content_name.unwrap();
            let restore_size = status.restore_size.unwrap();
            info!(
                "{}",
                format!(
                    "VolumeSnapshot is ready! VS name: {}, Bound VSC name: {}, Restore size: {}",
                    volume_snapshot_name, bound_vsc_name, restore_size
                )
            );
        }
        Ok(())
    }
}

/// A struct for holding the Kubernetes APIs for the backup operation
struct BackupKubernetesApisStruct {
    source_vs_api: Api<VolumeSnapshot>,
    source_pvcs_api: KubePvcApi,
    vsc_api: Api<VolumeSnapshotContent>,
}
