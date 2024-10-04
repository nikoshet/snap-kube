pub mod retain_policy;
pub mod volume_snapshot_contents;
#[cfg(feature = "restore")]
pub mod volume_snapshot_contents_operator;

#[cfg(test)]
mod volume_snapshot_contents_tests;
