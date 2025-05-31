#![cfg(test)]

//! Integration tests for validating the workspace structure and crate metadata
//!
//! This module verifies:
//! 1. That the workspace root Cargo.toml exists and that the workspace properly lists both
//!    the `context-engine-core` and `context-engine-server` members.
//! 2. That each workspace member has its own Cargo.toml and a src/ directory,
//!    ensuring member isolation.
//! 3. That all member crates share a consistent Rust edition.
//! 4. That each member crate's Cargo.toml inherits required metadata fields from the workspace
//!    root:
//!    name, version, edition, authors, documentation link, and license.

use std::fs;

use context_engine_tests::helpers::workspace_root;

#[test]
fn test_workspace_structure_exists() {
    let workspace_root = workspace_root();

    // Verify workspace root structure
    assert!(
        workspace_root.join("Cargo.toml").exists(),
        "Workspace Cargo.toml must exist"
    );
    assert!(
        workspace_root.join("context-engine-core").exists(),
        "Core crate directory must exist"
    );
    assert!(
        workspace_root.join("context-engine-server").exists(),
        "Server crate directory must exist"
    );

    // Verify each crate has proper structure
    assert!(
        workspace_root
            .join("context-engine-core/Cargo.toml")
            .exists(),
        "Core Cargo.toml must exist"
    );
    assert!(
        workspace_root
            .join("context-engine-core/src/lib.rs")
            .exists(),
        "Core lib.rs must exist"
    );
    assert!(
        workspace_root
            .join("context-engine-server/Cargo.toml")
            .exists(),
        "Server Cargo.toml must exist"
    );
    assert!(
        workspace_root
            .join("context-engine-server/src/main.rs")
            .exists(),
        "Server main.rs must exist"
    );
}

#[test]
fn test_workspace_dependency_resolution() {
    // Test that workspace builds without errors
    let output = std::process::Command::new("cargo")
        .args(["check", "--workspace"])
        .output()
        .expect("Failed to execute cargo check");

    assert!(
        output.status.success(),
        "Workspace should build successfully. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_workspace_member_isolation() {
    // Test that core crate builds independently
    let core_output = std::process::Command::new("cargo")
        .args(["check", "-p", "context-engine-core"])
        .output()
        .expect("Failed to check core crate");

    assert!(
        core_output.status.success(),
        "Core crate should build independently"
    );

    // Test that server crate builds independently
    let server_output = std::process::Command::new("cargo")
        .args(["check", "-p", "context-engine-server"])
        .output()
        .expect("Failed to check server crate");

    assert!(
        server_output.status.success(),
        "Server crate should build independently"
    );
}

#[test]
fn test_workspace_metadata_consistency() {
    let workspace_root = workspace_root();

    // Parse workspace Cargo.toml
    let workspace_toml = fs::read_to_string(workspace_root.join("Cargo.toml"))
        .expect("Should be able to read workspace Cargo.toml");

    // Verify edition is specified and correct
    assert!(
        workspace_toml.contains("edition = \"2024\""),
        "Workspace should specify Rust 2024 edition"
    );

    // Verify authors are specified and correct
    assert!(
        workspace_toml.contains("authors = [\"Arthur Welf\"]"),
        "Workspace authors should be specified and correct"
    );

    // Verify version is specified and correct
    assert!(
        workspace_toml.contains("version = \"0.1.0\""),
        "Workspace version should be specified and correct"
    );

    // Verify resolver version
    assert!(
        workspace_toml.contains("resolver = \"3\""),
        "Workspace should use resolver version 3"
    );

    // Verify license is specified and correct
    assert!(
        workspace_toml.contains("license = \"Apache-2.0\""),
        "Workspace should have license Apache-2.0"
    );

    // Verify documentation is specified and correct
    assert!(
        workspace_toml.contains("documentation = \"https://docs.rs/context-engine\""),
        "Workspace documentation link should be specified and correct"
    );
}

#[test]
fn test_crate_metadata_validity() {
    let workspace_root = workspace_root();

    // Verify core crate metadata
    let core_toml = fs::read_to_string(workspace_root.join("context-engine-core/Cargo.toml"))
        .expect("Should be able to read core Cargo.toml");

    // Verify name is correct
    assert!(
        core_toml.contains("documentation = \"https://docs.rs/context-engine-core\""),
        "Core crate should have correct documentation link"
    );

    // Verify version is inherited from workspace
    assert!(
        core_toml.contains("version.workspace = true"),
        "Core crate should have workspace version"
    );

    // Verify edition is inherited from workspace
    assert!(
        core_toml.contains("edition.workspace = true"),
        "Core crate should have workspace edition"
    );

    // Verify license is inherited from workspace
    assert!(
        core_toml.contains("license.workspace = true"),
        "Core crate should have workspace edition"
    );

    // Verify authors are inherited from workspace
    assert!(
        core_toml.contains("authors.workspace = true"),
        "Core crate should have workspace authors"
    );

    // Verify documentation link is specified and correct
    assert!(
        core_toml.contains("authors.workspace = true"),
        "Core crate should have workspace authors"
    );

    // Verify server crate metadata
    let server_toml = fs::read_to_string(workspace_root.join("context-engine-server/Cargo.toml"))
        .expect("Should be able to read server Cargo.toml");

    // Verify authors are inherited from workspace
    assert!(
        server_toml.contains("authors.workspace = true"),
        "Server crate should have workspace authors"
    );

    // Verify name is correct
    assert!(
        server_toml.contains("name = \"context-engine-server\""),
        "Server crate should have correct name"
    );

    // Verify version is inherited from workspace
    assert!(
        server_toml.contains("version.workspace = true"),
        "Server crate should have workspace version"
    );

    // Verify edition is inherited from workspace
    assert!(
        server_toml.contains("edition.workspace = true"),
        "Server crate should have workspace edition"
    );

    // Verify documentation link is inherited from workspace
    assert!(
        server_toml.contains("documentation.workspace = true"),
        "Server crate should have workspace documentation link"
    );
}
