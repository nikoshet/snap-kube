<h1 align="center">SnapKube</h1>
<div align="center">
 <strong>
   A Rust 🦀 tool supports (for now) PVC snapshots across Kubernetes namespaces
 </strong>
</div>

<br />

<div align="center">
  <!-- Github Actions -->
  <a href="https://github.com/nikoshet/snap-kube/actions/workflows/ci.yaml?query=branch%3Amain">
    <img src="https://img.shields.io/github/actions/workflow/status/nikoshet/snap-kube/ci.yaml?branch=main&style=flat-square" alt="actions status" /></a>
  <!-- Version -->
  <a href="https://crates.io/crates/snap-kube">
    <img src="https://img.shields.io/crates/v/snap-kube.svg?style=flat-square"
    alt="Crates.io version" /></a>
  <!-- Docs -->
  <a href="https://docs.rs/snap-kube">
  <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/snap-kube">
    <img src="https://img.shields.io/crates/d/snap-kube.svg?style=flat-square" alt="Download" />
  </a>
</div>

## Table of Contents
1. [Overview](#Overview)
2. [Modes of Operation](#modes-of-operation)
2. [Features](#Features)
3. [Prerequisites](#Prerequisites)
4. [Installation](#Installation)
    - [Client](#client)
    - [Library](#Library)
5. [Example](#Example)
6. [Tested Versions](#Tested-Versions)
7. [License](#License)

## Overview
The SnapKube Tool is a Rust-based utility that allows Kubernetes users to backup and restore Persistent Volume Claim (PVC) snapshots. The tool provides robust mechanisms to back up data to AWS Elastic Block Store (EBS) and restore it to any Kubernetes namespace. This tool is designed to work with Kubernetes VolumeSnapshot resources, making backup and restoration operations seamless.


## Modes of Operation

The tool supports three primary modes of operation:

| Mode    | Description                                      |
|---------|--------------------------------------------------|
| Backup  | Create snapshots of one or more PVCs.            |
| Restore | Restore PVCs from existing snapshots.            |
| Full    | Run both backup and restore operations in a single process. |


## Features
- **Backup**: Create Kubernetes VolumeSnapshots from existing PVCs
- **Restore**: Restore PVCs to any namespace from a VolumeSnapshot
- **Flexible Configuration**: The user can either snapshot a specific PVC, or all the PVCs in a specific namespace using the relative flags
- **AWS EBS Integration**: Natively supports backup and restoration to AWS Elastic Block Store
- **Conditional Compilation**: Enable or disable specific modes (backup, restore, full) via Rust feature flags, optimizing binary size and performance.
- **Error Handling**: Robust error handling and retries to ensure operations complete reliably

## Prerequisites
Before using SnapKube, please ensure you have the following:
- You need Rust installed to compile the tool. Install Rust via rustup
- An AWS Account with the appropriate access policy
- AWS EBS CSI Driver: Required to be installed in your Kubernetes cluster, which is a CSI Driver to manage the lifecycle of EBS Volumes
- CSI Snapshot Controller: A snapshot-controller that supports handling the VolumeSnapshot and VolumeSnapshotContent Objects
- A specific VolumeSnapshotClass for the CSI driver
- Kubernetes CLI

## Installation 

### Client
In order to use the tool as a client, you can use `cargo`.

```
cargo install snap-kube-client
```

The tool provides 3 features for running it, which are `backup` `restore`, and `full` (default).
```shell
Usage: snap-kube-client full [OPTIONS] --source-ns <SOURCE_NS> --target-ns <TARGET_NS> --volume-snapshot-class <VOLUME_SNAPSHOT_CLASS> --volume-snapshot-name-prefix <VOLUME_SNAPSHOT_NAME_PREFIX> --target-snapshot-content-name-prefix <TARGET_SNAPSHOT_CONTENT_NAME_PREFIX> --storage-class-name <STORAGE_CLASS_NAME>

Options:
      --region <REGION>
          Region where the EBS volumes are stored [default: eu-west-1]
      --source-ns <SOURCE_NS>
          Source namespace
      --target-ns <TARGET_NS>
          Target namespace
      --volume-snapshot-class <VOLUME_SNAPSHOT_CLASS>
          VolumeSnapshotClass name
      --pvc-name <PVC_NAME>
          PVC name [default: ]
      --include-all-pvcs
          Include all PVCs in the namespace
      --volume-snapshot-name-prefix <VOLUME_SNAPSHOT_NAME_PREFIX>
          VolumeSnapshot name prefix
      --target-snapshot-content-name-prefix <TARGET_SNAPSHOT_CONTENT_NAME_PREFIX>
          Target VolumeSnapshotContent name prefix
      --storage-class-name <STORAGE_CLASS_NAME>
          StorageClass name
      --vsc-retain-policy <VSC_RETAIN_POLICY>
          VSC Retain Policy [default: delete] [possible values: retain, delete]
  -h, --help
          Print help
  -V, --version
          Print version
```

### Library
To install the tool as a library, you can add it to your `Cargo.toml`:
```
cargo add snap-kube
```
or
```
[dependencies]
snap-client = "0.X"
```

Run the tool:
- For **full** mode:
```
cargo run full
```

- For **backup** mode:
```
cargo run --no-default-features --features backup -- backup
```

- For **restore** mode:
```
cargo run --no-default-features --features restore -- restore
```

## Example

- Build and run the Rust tool
```shell
cargo fmt --all
cargo clippy --all
cargo nextest run --all
cargo build

RUST_LOG=info \
    cargo run full \
    --source-ns "source-ns" \
    --target-ns "target-ns" \
    --volume-snapshot-class "volumesnapshotclass-name" \
    --include-all-pvcs \
    --volume-snapshot-name-prefix "prefix-vs" \
    --target-snapshot-content-name-prefix "prefix-vsc" \
    --storage-class-name "ebs-test-sc"
```

## Tested Versions

- Kubernetes v1.30
- Rust v1.81.0
- Amazon EBS CSI Driver v1.35.0-eksbuild.1
- CSI Snapshot Controller: v8.0.0-eksbuild.1

## License
This project is licensed under the MIT License 
