use super::persistent_volume_claims_payload::PVCOperatorPayload;
use k8s_openapi::{
    api::core::v1::{
        PersistentVolumeClaim, PersistentVolumeClaimSpec, TypedLocalObjectReference,
        TypedObjectReference, VolumeResourceRequirements,
    },
    apimachinery::pkg::api::resource::Quantity,
};
use kube::api::ObjectMeta;
use std::collections::BTreeMap;

enum PVCResourceValues {
    AccessModes,
    K8sKind,
    ApiGroup,
    VolumeMode,
    StorageClass,
}

impl PVCResourceValues {
    pub fn get_value(&self) -> String {
        match self {
            PVCResourceValues::AccessModes => "ReadWriteOnce".to_string(),
            PVCResourceValues::K8sKind => "VolumeSnapshot".to_string(),
            PVCResourceValues::ApiGroup => "snapshot.storage.k8s.io".to_string(),
            PVCResourceValues::VolumeMode => "Filesystem".to_string(),
            PVCResourceValues::StorageClass => "gp3".to_string(),
        }
    }
}

pub struct PVCOperator {
    pvc_operator_payload: PVCOperatorPayload,
}

impl PVCOperator {
    pub fn new(pvc_operator_payload: PVCOperatorPayload) -> Self {
        Self {
            pvc_operator_payload,
        }
    }

    /// Construct a PersistentVolumeClaim resource
    ///
    /// # Arguments
    ///
    /// * `pvc_name` - Name of the PersistentVolumeClaim resource
    /// * `namespace` - Namespace of the PersistentVolumeClaim resource
    /// * `storage_class` - Name of the StorageClass resource
    /// * `access_modes` - Access modes for the PersistentVolumeClaim resource
    /// * `volume_snapshot_name` - Name of the VolumeSnapshot resource
    /// * `restore_size` - Size of the PersistentVolumeClaim resource
    ///
    /// # Returns
    ///
    /// PersistentVolumeClaim resource
    pub fn construct_persistent_volume_claim_resource(&self) -> PersistentVolumeClaim {
        // Create a base labels map
        // Always add the VSc name
        let labels = BTreeMap::from([(
            "snap-kube/volume-snapshot-name".to_string(),
            self.pvc_operator_payload.pvc_name().to_string(),
        )]);

        PersistentVolumeClaim {
            metadata: ObjectMeta {
                name: Some(String::from(self.pvc_operator_payload.pvc_name())),
                namespace: Some(String::from(self.pvc_operator_payload.namespace())),
                labels: Some(labels),
                ..Default::default()
            },
            spec: Some(PersistentVolumeClaimSpec {
                access_modes: Some(
                    self.pvc_operator_payload
                        .access_modes()
                        .unwrap_or(vec![PVCResourceValues::AccessModes.get_value()]),
                ),
                storage_class_name: Some(
                    self.pvc_operator_payload
                        .storage_class()
                        .unwrap_or(&PVCResourceValues::StorageClass.get_value())
                        .to_string(),
                ),
                data_source: Some(TypedLocalObjectReference {
                    name: String::from(self.pvc_operator_payload.volume_snapshot_name()),
                    kind: PVCResourceValues::K8sKind.get_value(),
                    api_group: Some(PVCResourceValues::ApiGroup.get_value()),
                }),
                data_source_ref: Some(TypedObjectReference {
                    name: String::from(self.pvc_operator_payload.volume_snapshot_name()),
                    kind: PVCResourceValues::K8sKind.get_value(),
                    api_group: Some(PVCResourceValues::ApiGroup.get_value()),
                    namespace: Some(String::from(self.pvc_operator_payload.namespace())),
                }),
                volume_mode: Some(PVCResourceValues::VolumeMode.get_value()),
                volume_name: Default::default(),
                resources: Some(VolumeResourceRequirements {
                    requests: Some(BTreeMap::from([(
                        "storage".to_string(),
                        Quantity(String::from(self.pvc_operator_payload.restore_size())),
                    )])),
                    ..Default::default()
                }),
                selector: Default::default(),
                volume_attributes_class_name: Default::default(),
            }),
            ..Default::default()
        }
    }
}
