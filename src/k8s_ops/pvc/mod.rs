pub mod persistent_volume_claims;
#[cfg(feature = "restore")]
pub mod persistent_volume_claims_operator;
#[cfg(feature = "restore")]
pub mod persistent_volume_claims_payload;

#[cfg(test)]
mod persistent_volume_claims_tests;
