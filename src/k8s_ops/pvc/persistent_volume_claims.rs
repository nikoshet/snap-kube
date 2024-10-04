use anyhow::Result;
use async_trait::async_trait;
use k8s_openapi::api::core::v1::PersistentVolumeClaim;
use kube::{api::ListParams, Api};
use tracing::info;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait PvcApiTrait {
    async fn list_pvcs(&self) -> Result<Vec<PersistentVolumeClaim>>;
    async fn get(&self, name: &str) -> Result<PersistentVolumeClaim>;
    async fn create(&self, pvc: PersistentVolumeClaim) -> Result<PersistentVolumeClaim>;
}

pub struct KubePvcApi {
    pub api: Api<PersistentVolumeClaim>,
}

/// Implement the PvcApi trait for PVC Api
/// This will allow us to call the functions defined in the PvcApi trait on an instance of KubePvcApi.
/// This is useful for testing, as we can mock the KubePvcApi struct and implement the PvcApi trait
/// to return mock data.
/// This way, we can test the functions that use the KubePvcApi struct without actually making
/// calls to the Kubernetes API.
#[async_trait]
impl PvcApiTrait for KubePvcApi {
    async fn list_pvcs(&self) -> Result<Vec<PersistentVolumeClaim>> {
        let pvcs = self.api.list(&ListParams::default()).await?;
        Ok(pvcs.items)
    }

    async fn get(&self, name: &str) -> Result<PersistentVolumeClaim> {
        let pvc = self.api.get(name).await?;
        Ok(pvc)
    }

    async fn create(&self, pvc: PersistentVolumeClaim) -> Result<PersistentVolumeClaim> {
        let pvc = self.api.create(&Default::default(), &pvc).await?;
        Ok(pvc)
    }
}

/// Get the list of PersistentVolumeClaims available
pub async fn get_pvcs_available(pvc_api: &impl PvcApiTrait) -> Result<Vec<String>> {
    let pvc_list: Vec<_> = match pvc_api.list_pvcs().await {
        Ok(pvc) => pvc,
        Err(e) => panic!("Failed to list PVCs: {}", e),
    }
    .into_iter()
    .map(|pvc| pvc.metadata.name.unwrap())
    .collect();
    info!("PVCs available: {:?}", pvc_list);
    Ok(pvc_list)
}

pub async fn check_if_pvc_exists(
    target_pvc_api: &impl PvcApiTrait,
    //&kube::Api<PersistentVolumeClaim>,
    pvc_name: &str,
    should_exist: bool,
) -> Result<Option<PersistentVolumeClaim>> {
    match target_pvc_api.get(pvc_name).await {
        Ok(pvc) => {
            if should_exist {
                info!(
                    "{}",
                    format!(
                        "PVC exists: {} on target namespace {:?}",
                        pvc.metadata.name.clone().unwrap(),
                        pvc.metadata.namespace.clone().unwrap()
                    )
                );
                Ok(Some(pvc))
            } else {
                panic!(
                    "PVC does not exist: {} on target namespace {:?}",
                    pvc.metadata.name.clone().unwrap(),
                    pvc.metadata.namespace.clone().unwrap()
                );
            }
        }
        Err(e) => {
            if should_exist {
                panic!("Failed to get PVC: {}", e);
            } else {
                info!("PVC does not exist: {}", pvc_name);
                Ok(None)
            }
        }
    }
}
