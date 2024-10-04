use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
#[cfg(feature = "backup")]
use snap_kube::backup::{backup_operator::BackupOperator, backup_payload::BackupPayload};
#[cfg(feature = "restore")]
use snap_kube::k8s_ops::vsc::retain_policy::VSCRetainPolicy;
#[cfg(feature = "restore")]
use snap_kube::restore::{restore_operator::RestoreOperator, restore_payload::RestorePayload};
use tracing::info;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "backup")]
    Backup {
        /// Region where the EBS volumes are stored
        #[arg(long, required = false, default_value = "eu-west-1")]
        region: String,
        /// Source namespace
        #[arg(long, required = true)]
        source_ns: String,
        /// VolumeSnapshotClass name
        #[arg(long, required = true)]
        volume_snapshot_class: String,
        /// PVC name
        #[arg(long, required = false, conflicts_with = "include_all_pvcs")]
        pvc_name: Option<String>,
        /// Include all PVCs in the namespace
        #[arg(
            long,
            required = false,
            default_value = "false",
            conflicts_with = "pvc_name"
        )]
        include_all_pvcs: bool,
        /// VolumeSnapshot name prefix
        #[arg(long, required = true)]
        volume_snapshot_name_prefix: String,
    },
    #[cfg(feature = "restore")]
    Restore {
        /// Source namespace
        #[arg(long, required = true)]
        source_ns: String,
        /// Target namespace
        #[arg(long, required = true)]
        target_ns: String,
        /// VolumeSnapshotClass name
        #[arg(long, required = true)]
        volume_snapshot_class: String,
        /// PVC name
        #[arg(long, required = false, conflicts_with = "include_all_pvcs")]
        pvc_name: Option<String>,
        /// Include all PVCs in the namespace
        #[arg(
            long,
            required = false,
            default_value = "false",
            conflicts_with = "pvc_name"
        )]
        include_all_pvcs: bool,
        /// VolumeSnapshot name prefix
        #[arg(long, required = true)]
        volume_snapshot_name_prefix: String,
        /// Target VolumeSnapshotContent name prefix
        #[arg(long, required = true)]
        target_snapshot_content_name_prefix: String,
        /// StorageClass name
        #[arg(long, required = true)]
        storage_class_name: String,
        /// VSC Retain Policy
        #[arg(long, required = false, default_value = "delete")]
        #[clap(value_enum)]
        vsc_retain_policy: VSCRetainPolicy,
    },
    #[cfg(feature = "full")]
    Full {
        /// Region where the EBS volumes are stored
        #[arg(long, required = false, default_value = "eu-west-1")]
        region: String,
        /// Source namespace
        #[arg(long, required = true)]
        source_ns: String,
        /// Target namespace
        #[arg(long, required = true)]
        target_ns: String,
        /// VolumeSnapshotClass name
        #[arg(long, required = true)]
        volume_snapshot_class: String,
        /// PVC name
        #[arg(long, required = false, conflicts_with = "include_all_pvcs")]
        pvc_name: Option<String>,
        /// Include all PVCs in the namespace
        #[arg(
            long,
            required = false,
            default_value = "false",
            conflicts_with = "pvc_name"
        )]
        include_all_pvcs: bool,
        /// VolumeSnapshot name prefix
        #[arg(long, required = true)]
        volume_snapshot_name_prefix: String,
        /// Target VolumeSnapshotContent name prefix
        #[arg(long, required = true)]
        target_snapshot_content_name_prefix: String,
        /// StorageClass name
        #[arg(long, required = true)]
        storage_class_name: String,
        /// VSC Retain Policy
        #[arg(long, required = false, default_value = "delete")]
        #[clap(value_enum)]
        vsc_retain_policy: VSCRetainPolicy,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    match cli.command {
        #[cfg(feature = "backup")]
        Commands::Backup {
            region,
            source_ns,
            volume_snapshot_class,
            pvc_name,
            include_all_pvcs,
            volume_snapshot_name_prefix,
        } => {
            let backup_payload = BackupPayload::new(
                region,
                source_ns,
                volume_snapshot_class,
                pvc_name,
                include_all_pvcs,
                volume_snapshot_name_prefix,
            );

            info!("{}", "Starting Backup process...".bold().blue());
            BackupOperator::backup(backup_payload).await?;
            info!(
                "{}",
                "Backup process completed successfully!".bold().green()
            );
        }
        #[cfg(feature = "restore")]
        Commands::Restore {
            source_ns,
            target_ns,
            volume_snapshot_class,
            pvc_name,
            include_all_pvcs,
            volume_snapshot_name_prefix,
            target_snapshot_content_name_prefix,
            storage_class_name,
            vsc_retain_policy,
        } => {
            let restore_payload = RestorePayload::new(
                source_ns.clone(),
                target_ns.clone(),
                volume_snapshot_class.clone(),
                pvc_name.clone(),
                include_all_pvcs,
                volume_snapshot_name_prefix.clone(),
                target_snapshot_content_name_prefix.clone(),
                storage_class_name.clone(),
                vsc_retain_policy,
            );
            info!("{}", "Starting Restore process...".bold().blue());
            RestoreOperator::restore(restore_payload).await?;
            info!(
                "{}",
                "Restore process completed successfully!".bold().green()
            );
        }
        #[cfg(feature = "full")]
        Commands::Full {
            region,
            source_ns,
            target_ns,
            volume_snapshot_class,
            pvc_name,
            include_all_pvcs,
            volume_snapshot_name_prefix,
            target_snapshot_content_name_prefix,
            storage_class_name,
            vsc_retain_policy,
        } => {
            let backup_payload = BackupPayload::new(
                region.clone(),
                source_ns.clone(),
                volume_snapshot_class.clone(),
                pvc_name.clone(),
                include_all_pvcs,
                volume_snapshot_name_prefix.clone(),
            );

            let restore_payload = RestorePayload::new(
                source_ns.clone(),
                target_ns.clone(),
                volume_snapshot_class.clone(),
                pvc_name.clone(),
                include_all_pvcs,
                volume_snapshot_name_prefix.clone(),
                target_snapshot_content_name_prefix.clone(),
                storage_class_name.clone(),
                vsc_retain_policy,
            );

            info!("{}", "Starting Backup process...".bold().blue());
            BackupOperator::backup(backup_payload).await?;
            info!(
                "{}",
                "Backup process completed successfully!".bold().green()
            );

            info!("{}", "Starting Restore process...".bold().blue());
            RestoreOperator::restore(restore_payload).await?;
            info!(
                "{}",
                "Restore process completed successfully!".bold().green()
            );
        }
    };
    Ok(())
}
