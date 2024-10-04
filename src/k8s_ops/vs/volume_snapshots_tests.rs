#[cfg(test)]
mod tests {
    use crate::k8s_ops::{
        vs::volume_snapshots_operator::VolumeSnapshotOperator, vsc::retain_policy::VSCRetainPolicy,
    };
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_construct_volume_snapshot_resource() {
        let vs_operator = VolumeSnapshotOperator::new(
            "test-volume-snapshot".to_string(),
            "default".to_string(),
            "ebs.csi.aws.com".to_string(),
            Some("test-pvc".to_string()),
            Some("test-volume-snapshot-content".to_string()),
        );
        let volume_snapshot = vs_operator.construct_volume_snapshot_resource(
            Some("test-snapshot-handle".to_string()),
            Some("1Gi".to_string()),
            VSCRetainPolicy::Delete,
        );
        assert_eq!(
            volume_snapshot.metadata.name.unwrap(),
            "test-volume-snapshot"
        );
        assert_eq!(volume_snapshot.metadata.namespace.unwrap(), "default");
        assert_eq!(
            volume_snapshot
                .metadata
                .annotations
                .as_ref()
                .unwrap()
                .get("snap-kube/csi-driver-name"),
            Some(&"ebs.csi.aws.com".to_string())
        );
        assert_eq!(
            volume_snapshot
                .metadata
                .annotations
                .as_ref()
                .unwrap()
                .get("snap-kube/csi-vsc-deletion-policy"),
            Some(&"Delete".to_string())
        );
        assert_eq!(
            volume_snapshot
                .metadata
                .annotations
                .as_ref()
                .unwrap()
                .get("snap-kube/csi-volumesnapshot-handle"),
            Some(&"test-snapshot-handle".to_string())
        );
        assert_eq!(
            volume_snapshot
                .metadata
                .annotations
                .unwrap()
                .get("snap-kube/csi-volumesnapshot-restore-size"),
            Some(&"1Gi".to_string())
        );
        assert_eq!(
            volume_snapshot
                .metadata
                .labels
                .unwrap()
                .get("app.kubernetes.io/instance"),
            Some(&"default".to_string())
        );
        assert_eq!(
            volume_snapshot.spec.volume_snapshot_class_name.unwrap(),
            "ebs.csi.aws.com"
        );
        assert_eq!(
            volume_snapshot
                .spec
                .source
                .persistent_volume_claim_name
                .unwrap(),
            "test-pvc"
        );
        assert_eq!(
            volume_snapshot
                .spec
                .source
                .volume_snapshot_content_name
                .unwrap(),
            "test-volume-snapshot-content"
        );
    }
}
