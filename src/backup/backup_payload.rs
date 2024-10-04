pub struct BackupPayload {
    pub region: String,
    pub source_ns: String,
    pub volume_snapshot_class: String,
    pub pvc_name: Option<String>,
    pub include_all_pvcs: bool,
    pub vs_name_prefix: String,
}

impl BackupPayload {
    /// Creates a new BackupPayload
    ///
    /// # Arguments
    ///
    /// * `region` - AWS region
    /// * `source_ns` - Source namespace
    /// * `volume_snapshot_class` - VolumeSnapshotClass name
    /// * `pvc_name` - PVC name
    /// * `include_all_pvcs` - Include all PVCs in the namespace
    /// * `vs_name_prefix` - VolumeSnapshot name prefix
    ///
    /// # Returns
    ///
    /// A new BackupPayload instance
    pub fn new(
        region: impl Into<String>,
        source_ns: impl Into<String>,
        volume_snapshot_class: impl Into<String>,
        pvc_name: Option<impl Into<String>>,
        include_all_pvcs: bool,
        vs_name_prefix: impl Into<String>,
    ) -> Self {
        Self {
            region: region.into(),
            source_ns: source_ns.into(),
            volume_snapshot_class: volume_snapshot_class.into(),
            pvc_name: pvc_name.map(|pvc_name| pvc_name.into()),
            include_all_pvcs,
            vs_name_prefix: vs_name_prefix.into(),
        }
    }

    pub fn region(&self) -> &str {
        &self.region
    }

    pub fn source_ns(&self) -> &str {
        &self.source_ns
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
}
