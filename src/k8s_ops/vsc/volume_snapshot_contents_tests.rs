#[cfg(test)]
mod tests {
    use crate::k8s_ops::vsc::{
        retain_policy::VSCRetainPolicy,
        volume_snapshot_contents_operator::VolumeSnapshotContentOperator,
    };

    #[tokio::test]
    async fn test_construct_volume_snapshot_content_resource() {
        let vsc_operator = VolumeSnapshotContentOperator::new(
            "test-volume-snapshot-content".to_string(),
            "default".to_string(),
            "test-volume-snapshot".to_string(),
            Some("ebs.csi.aws.com".to_string()),
            Some("test-snapshot-handle".to_string()),
            VSCRetainPolicy::Delete,
        );
        let volume_snapshot_content = vsc_operator.construct_volume_snapshot_content_resource();
        assert_eq!(
            volume_snapshot_content.metadata.name.unwrap(),
            "test-volume-snapshot-content"
        );
        assert_eq!(
            volume_snapshot_content.metadata.namespace.unwrap(),
            "default"
        );
        assert_eq!(
            volume_snapshot_content
                .spec
                .volume_snapshot_ref
                .name
                .unwrap(),
            "test-volume-snapshot"
        );
        assert_eq!(
            volume_snapshot_content
                .spec
                .volume_snapshot_class_name
                .unwrap(),
            "ebs.csi.aws.com"
        );
        assert_eq!(
            volume_snapshot_content.spec.source.snapshot_handle.unwrap(),
            "test-snapshot-handle"
        );
        assert_eq!(
            volume_snapshot_content.spec.source_volume_mode.unwrap(),
            "Filesystem"
        );
        assert_eq!(
            volume_snapshot_content
                .status
                .as_ref()
                .unwrap()
                .snapshot_handle
                .as_ref()
                .unwrap(),
            "test-snapshot-handle"
        );
    }
}
