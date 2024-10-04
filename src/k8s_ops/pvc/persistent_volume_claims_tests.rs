#[cfg(test)]
mod tests {
    use crate::k8s_ops::pvc::{
        persistent_volume_claims::{MockPvcApiTrait, PvcApiTrait},
        persistent_volume_claims_operator::PVCOperator,
        persistent_volume_claims_payload::PVCOperatorPayload,
    };
    use k8s_openapi::{
        api::core::v1::{
            PersistentVolumeClaim, PersistentVolumeClaimSpec, TypedLocalObjectReference,
            TypedObjectReference, VolumeResourceRequirements,
        },
        apimachinery::pkg::api::resource::Quantity,
    };
    use kube::api::ObjectMeta;
    use mockall::predicate;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn test_construct_persistent_volume_claim_resource() {
        let pvc_operator_payload = PVCOperatorPayload::new(
            String::from("test-pvc"),
            String::from("test-ns"),
            String::from("gp3"),
            Some(vec![String::from("ReadWriteOnce")]),
            String::from("test-vs"),
            "1Gi".to_string(),
        );

        let pvc_operator = PVCOperator::new(pvc_operator_payload);

        let pvc = pvc_operator.construct_persistent_volume_claim_resource();

        let labels = BTreeMap::from([(
            "snap-kube/volume-snapshot-name".to_string(),
            "test-pvc".to_string(),
        )]);

        let expected_pvc = PersistentVolumeClaim {
            metadata: ObjectMeta {
                name: Some("test-pvc".to_string()),
                namespace: Some("test-ns".to_string()),
                labels: Some(labels),
                ..Default::default()
            },
            spec: Some(PersistentVolumeClaimSpec {
                access_modes: Some(vec!["ReadWriteOnce".to_string()]),
                storage_class_name: Some("gp3".to_string()),
                data_source: Some(TypedLocalObjectReference {
                    name: "test-vs".to_string(),
                    kind: "VolumeSnapshot".to_string(),
                    api_group: Some("snapshot.storage.k8s.io".to_string()),
                }),
                data_source_ref: Some(TypedObjectReference {
                    name: "test-vs".to_string(),
                    kind: "VolumeSnapshot".to_string(),
                    api_group: Some("snapshot.storage.k8s.io".to_string()),
                    namespace: Some("test-ns".to_string()),
                }),
                volume_mode: Some("Filesystem".to_string()),
                volume_name: Default::default(),
                resources: Some(VolumeResourceRequirements {
                    requests: Some(BTreeMap::from([(
                        "storage".to_string(),
                        Quantity("1Gi".to_string()),
                    )])),
                    ..Default::default()
                }),
                selector: Default::default(),
                volume_attributes_class_name: Default::default(),
            }),
            ..Default::default()
        };

        assert_eq!(pvc, expected_pvc);
    }

    #[tokio::test]
    async fn test_create_pvc() {
        let mut mock_pvc_api = MockPvcApiTrait::new();

        let pvc = PersistentVolumeClaim {
            metadata: ObjectMeta {
                name: Some("test-pvc".to_string()),
                namespace: Some("test-ns".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };

        mock_pvc_api
            .expect_create()
            .with(predicate::eq(pvc.clone()))
            .times(1)
            .returning(|_| {
                Ok(PersistentVolumeClaim {
                    metadata: ObjectMeta {
                        name: Some("test-pvc".to_string()),
                        namespace: Some("test-ns".to_string()),
                        ..Default::default()
                    },
                    ..Default::default()
                })
            });

        let result = mock_pvc_api.create(pvc).await;
        assert!(result.is_ok());
        assert_eq!(
            result.as_ref().cloned().unwrap().metadata.name.unwrap(),
            "test-pvc"
        );
        assert_eq!(result.unwrap().metadata.namespace.unwrap(), "test-ns");
    }

    #[tokio::test]
    async fn test_get_pvc() {
        let mut mock_pvc_api = MockPvcApiTrait::new();

        mock_pvc_api
            .expect_get()
            .with(predicate::eq("test-pvc"))
            .times(1)
            .returning(|_| {
                Ok(PersistentVolumeClaim {
                    metadata: ObjectMeta {
                        name: Some("test-pvc".to_string()),
                        namespace: Some("test-ns".to_string()),
                        ..Default::default()
                    },
                    ..Default::default()
                })
            });

        let result = mock_pvc_api.get("test-pvc").await;
        assert!(result.is_ok());
        assert_eq!(
            result.as_ref().cloned().unwrap().metadata.name.unwrap(),
            "test-pvc"
        );
        assert_eq!(result.unwrap().metadata.namespace.unwrap(), "test-ns");
    }

    #[tokio::test]
    async fn test_list_pvcs() {
        let mut mock_pvc_api = MockPvcApiTrait::new();

        mock_pvc_api.expect_list_pvcs().times(1).returning(|| {
            Ok(vec![PersistentVolumeClaim {
                metadata: ObjectMeta {
                    name: Some("test-pvc".to_string()),
                    namespace: Some("test-ns".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            }])
        });

        let result = mock_pvc_api.list_pvcs().await;

        assert!(result.is_ok());
        assert_eq!(result.as_ref().unwrap().len(), 1);
        assert_eq!(
            result
                .as_ref()
                .unwrap()
                .get(0)
                .unwrap()
                .metadata
                .name
                .clone()
                .unwrap(),
            "test-pvc"
        );
        assert_eq!(
            result
                .unwrap()
                .get(0)
                .unwrap()
                .metadata
                .namespace
                .clone()
                .unwrap(),
            "test-ns"
        );
    }
}
