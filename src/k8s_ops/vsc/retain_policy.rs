use clap::ValueEnum;
use kube_custom_resources_rs::snapshot_storage_k8s_io::v1::volumesnapshotcontents::VolumeSnapshotContentDeletionPolicy;
use std::fmt::{self, Display, Formatter};

/// Represents the VolumeSnapshotContent Retain Policy
///
/// It can be either Retain or Delete
#[derive(ValueEnum, Clone, Debug, Copy, PartialEq, Eq)]
pub enum VSCRetainPolicy {
    Retain,
    Delete,
}

impl Display for VSCRetainPolicy {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            VSCRetainPolicy::Retain => write!(f, "Retain"),
            VSCRetainPolicy::Delete => write!(f, "Delete"),
        }
    }
}

impl From<VSCRetainPolicy> for VolumeSnapshotContentDeletionPolicy {
    fn from(vsc_retain_policy: VSCRetainPolicy) -> Self {
        match vsc_retain_policy {
            VSCRetainPolicy::Retain => VolumeSnapshotContentDeletionPolicy::Retain,
            VSCRetainPolicy::Delete => VolumeSnapshotContentDeletionPolicy::Delete,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vsc_retain_policy_display() {
        assert_eq!(VSCRetainPolicy::Retain.to_string(), "Retain");
        assert_eq!(VSCRetainPolicy::Delete.to_string(), "Delete");
    }

    #[test]
    fn test_vsc_retain_policy_into() {
        assert_eq!(
            VolumeSnapshotContentDeletionPolicy::from(VSCRetainPolicy::Retain),
            VolumeSnapshotContentDeletionPolicy::Retain
        );
        assert_eq!(
            VolumeSnapshotContentDeletionPolicy::from(VSCRetainPolicy::Delete),
            VolumeSnapshotContentDeletionPolicy::Delete
        );
    }
}
