use super::retain_policy::VSCRetainPolicy;
use kube::api::ObjectMeta;
use kube_custom_resources_rs::snapshot_storage_k8s_io::v1::volumesnapshotcontents::{
    VolumeSnapshotContent, VolumeSnapshotContentSource, VolumeSnapshotContentSpec,
    VolumeSnapshotContentStatus, VolumeSnapshotContentVolumeSnapshotRef,
};

enum VSCResourceValues {
    ApiVersion,
    Kind,
    Driver,
    SourceVolumeMode,
}

impl VSCResourceValues {
    pub fn get_value(&self) -> String {
        match self {
            VSCResourceValues::ApiVersion => "snapshot.storage.k8s.io/v1".to_string(),
            VSCResourceValues::Kind => "VolumeSnapshot".to_string(),
            VSCResourceValues::Driver => "ebs.csi.aws.com".to_string(),
            VSCResourceValues::SourceVolumeMode => "Filesystem".to_string(),
        }
    }
}

pub struct VolumeSnapshotContentOperator {
    pub name: String,
    pub namespace: String,
    pub volume_snapshot_name: String,
    pub volume_snapshot_class: Option<String>,
    pub source_volume_handle: Option<String>,
    pub vsc_retain_policy: VSCRetainPolicy,
}

impl VolumeSnapshotContentOperator {
    pub fn new(
        name: String,
        namespace: String,
        volume_snapshot_name: String,
        volume_snapshot_class: Option<String>,
        source_volume_handle: Option<String>,
        vsc_retain_policy: VSCRetainPolicy,
    ) -> Self {
        Self {
            name,
            namespace,
            volume_snapshot_name,
            volume_snapshot_class,
            source_volume_handle,
            vsc_retain_policy,
        }
    }

    /// Construct a VolumeSnapshotContent resource
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the VolumeSnapshotContent resource
    /// * `namespace` - Namespace of the VolumeSnapshotContent resource
    /// * `volume_snapshot_name` - Name of the VolumeSnapshot resource
    /// * `volume_snapshot_class` - Name of the VolumeSnapshotClass resource
    /// * `source_volume_handle` - Handle - Snapshot ID of the source volume
    ///
    /// # Returns
    ///
    /// VolumeSnapshotContent resource
    pub fn construct_volume_snapshot_content_resource(&self) -> VolumeSnapshotContent {
        VolumeSnapshotContent {
            metadata: ObjectMeta {
                name: Some(self.name.clone()),
                namespace: Some(self.namespace.clone()),
                ..Default::default()
            },
            spec: VolumeSnapshotContentSpec {
                volume_snapshot_ref: VolumeSnapshotContentVolumeSnapshotRef {
                    api_version: Some(VSCResourceValues::ApiVersion.get_value()),
                    kind: Some(VSCResourceValues::Kind.get_value()),
                    name: Some(self.volume_snapshot_name.clone()),
                    namespace: Some(self.namespace.clone()),
                    field_path: Default::default(),
                    resource_version: Default::default(),
                    uid: Default::default(),
                },
                deletion_policy: self.vsc_retain_policy.into(),
                driver: VSCResourceValues::Driver.get_value(),
                source: VolumeSnapshotContentSource {
                    snapshot_handle: self.source_volume_handle.clone(),
                    ..Default::default()
                },
                volume_snapshot_class_name: self.volume_snapshot_class.clone(),
                source_volume_mode: Some(VSCResourceValues::SourceVolumeMode.get_value()),
            },
            status: Some(VolumeSnapshotContentStatus {
                snapshot_handle: self.source_volume_handle.clone(),
                creation_time: Default::default(),
                ready_to_use: Default::default(),
                restore_size: Default::default(),
                error: Default::default(),
                volume_group_snapshot_handle: Default::default(),
            }),
        }
    }
}
