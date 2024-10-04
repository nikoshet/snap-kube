pub struct PVCOperatorPayload {
    pub pvc_name: String,
    pub namespace: String,
    pub storage_class: Option<String>,
    pub access_modes: Option<Vec<String>>,
    pub volume_snapshot_name: String,
    pub restore_size: String,
}

impl PVCOperatorPayload {
    /// Creates a new PVCOperatorPayload
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
    /// A new PVCOperatorPayload instance
    pub fn new(
        pvc_name: impl Into<String>,
        namespace: impl Into<String>,
        storage_class: impl Into<Option<String>>,
        access_modes: impl Into<Option<Vec<String>>>,
        volume_snapshot_name: impl Into<String>,
        restore_size: impl Into<String>,
    ) -> Self {
        Self {
            pvc_name: pvc_name.into(),
            namespace: namespace.into(),
            storage_class: storage_class.into(),
            access_modes: access_modes.into(),
            volume_snapshot_name: volume_snapshot_name.into(),
            restore_size: restore_size.into(),
        }
    }

    pub fn pvc_name(&self) -> &str {
        &self.pvc_name
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn storage_class(&self) -> Option<&str> {
        self.storage_class.as_deref()
    }

    pub fn access_modes(&self) -> Option<Vec<String>> {
        self.access_modes.clone()
    }

    pub fn volume_snapshot_name(&self) -> &str {
        &self.volume_snapshot_name
    }

    pub fn restore_size(&self) -> &str {
        &self.restore_size
    }
}
