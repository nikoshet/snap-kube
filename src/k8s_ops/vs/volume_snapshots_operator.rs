use std::collections::BTreeMap;

use crate::k8s_ops::vsc::retain_policy::VSCRetainPolicy;
use kube::api::ObjectMeta;
use kube_custom_resources_rs::snapshot_storage_k8s_io::v1::volumesnapshots::{
    VolumeSnapshot, VolumeSnapshotSource, VolumeSnapshotSpec,
};

enum VSResourceValues {
    Finalizers,
}

impl VSResourceValues {
    pub fn get_value(&self) -> String {
        match self {
            VSResourceValues::Finalizers => {
                "snapshot.storage.kubernetes.io/volumesnapshot-bound-protection".to_string()
            }
        }
    }
}

pub struct VolumeSnapshotOperator {
    pub name: String,
    pub namespace: String,
    pub volume_snapshot_class: String,
    pub source_pvc_name: Option<String>,
    pub vsc_name: Option<String>,
}

impl VolumeSnapshotOperator {
    pub fn new(
        name: String,
        namespace: String,
        volume_snapshot_class: String,
        source_pvc_name: Option<String>,
        vsc_name: Option<String>,
    ) -> Self {
        Self {
            name,
            namespace,
            volume_snapshot_class,
            source_pvc_name,
            vsc_name,
        }
    }

    /// Construct a VolumeSnapshot resource
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the VolumeSnapshot resource
    /// * `namespace` - Namespace of the VolumeSnapshot resource
    /// * `volume_snapshot_class` - Name of the VolumeSnapshotClass resource
    /// * `source_pvc_name` - Name of the PersistentVolumeClaim resource
    /// * `vsc_name` - Name of the VolumeSnapshotContent resource
    /// * `snapshot_handle` - Handle - Snapshot ID of the source volume
    /// * `restore_size` - Size of the restored volume
    ///
    /// # Returns
    ///
    /// VolumeSnapshot resource
    pub fn construct_volume_snapshot_resource(
        &self,
        snapshot_handle: Option<String>,
        restore_size: Option<String>,
        vsc_retain_policy: VSCRetainPolicy,
    ) -> VolumeSnapshot {
        // Create a base annotations map with always-included entries
        let mut annotations = BTreeMap::from([
            ("snap-kube/csi-driver-name".into(), "ebs.csi.aws.com".into()),
            (
                "snap-kube/csi-vsc-deletion-policy".into(),
                vsc_retain_policy.to_string(),
            ),
        ]);

        // If a snapshot handle is provided, insert the corresponding annotation
        if let Some(handle) = snapshot_handle {
            annotations.insert("snap-kube/csi-volumesnapshot-handle".into(), handle);
        }
        // If a restore size is provided, insert the corresponding annotation
        if let Some(size) = restore_size {
            annotations.insert("snap-kube/csi-volumesnapshot-restore-size".into(), size);
        }

        // Create a base labels map
        // Always add the namespace name
        let labels = BTreeMap::from([(
            "app.kubernetes.io/instance".to_string(),
            self.namespace.clone(),
        )]);

        VolumeSnapshot {
            metadata: ObjectMeta {
                name: Some(self.name.clone()),
                namespace: Some(self.namespace.clone()),
                annotations: Some(annotations),
                finalizers: Some(vec![VSResourceValues::Finalizers.get_value()]),
                labels: Some(labels),
                ..Default::default()
            },
            spec: VolumeSnapshotSpec {
                volume_snapshot_class_name: Some(self.volume_snapshot_class.clone()),
                source: VolumeSnapshotSource {
                    persistent_volume_claim_name: self.source_pvc_name.clone(),
                    volume_snapshot_content_name: self.vsc_name.clone(),
                },
            },
            ..Default::default()
        }
    }
}
