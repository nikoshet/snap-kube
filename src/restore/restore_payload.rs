use crate::k8s_ops::vsc::retain_policy::VSCRetainPolicy;

pub struct RestorePayload {
    pub source_ns: String,
    pub target_ns: String,
    pub volume_snapshot_class: String,
    pub pvc_name: Option<String>,
    pub include_all_pvcs: bool,
    pub vs_name_prefix: String,
    pub vsc_name_prefix: String,
    pub storage_class_name: String,
    pub vsc_retain_policy: VSCRetainPolicy,
}

impl RestorePayload {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_ns: impl Into<String>,
        target_ns: impl Into<String>,
        volume_snapshot_class: impl Into<String>,
        pvc_name: Option<impl Into<String>>,
        include_all_pvcs: bool,
        vs_name_prefix: impl Into<String>,
        vsc_name_prefix: impl Into<String>,
        storage_class_name: impl Into<String>,
        vsc_retain_policy: VSCRetainPolicy,
    ) -> Self {
        Self {
            source_ns: source_ns.into(),
            target_ns: target_ns.into(),
            volume_snapshot_class: volume_snapshot_class.into(),
            pvc_name: pvc_name.map(|pvc_name| pvc_name.into()),
            include_all_pvcs,
            vs_name_prefix: vs_name_prefix.into(),
            vsc_name_prefix: vsc_name_prefix.into(),
            storage_class_name: storage_class_name.into(),
            vsc_retain_policy,
        }
    }

    pub fn source_ns(&self) -> &str {
        &self.source_ns
    }

    pub fn target_ns(&self) -> &str {
        &self.target_ns
    }

    pub fn volume_snapshot_class(&self) -> &str {
        &self.volume_snapshot_class
    }

    pub fn pvc_name(&self) -> Option<&str> {
        self.pvc_name.as_deref()
    }

    pub fn include_all_pvcs(&self) -> bool {
        self.include_all_pvcs
    }

    pub fn vs_name_prefix(&self) -> &str {
        &self.vs_name_prefix
    }

    pub fn vsc_name_prefix(&self) -> &str {
        &self.vsc_name_prefix
    }

    pub fn storage_class_name(&self) -> &str {
        &self.storage_class_name
    }

    pub fn vsc_retain_policy(&self) -> &VSCRetainPolicy {
        &self.vsc_retain_policy
    }
}
