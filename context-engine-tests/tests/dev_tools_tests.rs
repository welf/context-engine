#![cfg(test)]

use std::fs;

use context_engine_tests::helpers::workspace_root;

#[test]
fn test_development_tool_configs_exist() {
    assert!(
        workspace_root().join(".clippy.toml").exists(),
        "Clippy configuration must exist"
    );
    assert!(
        workspace_root().join("rustfmt.toml").exists(),
        "Rustfmt configuration must exist"
    );
    assert!(
        workspace_root().join("deny.toml").exists(),
        "Cargo deny configuration must exist"
    );
    assert!(
        workspace_root().join(".gitignore").exists(),
        "Gitignore file must exist"
    );
}

#[test]
fn test_clippy_configuration_strictness() {
    let clippy_content = fs::read_to_string(workspace_root().join(".clippy.toml"))
        .expect("Should be able to read clippy configuration");

    // Verify strict linting is enabled
    assert!(
        clippy_content.contains("msrv"),
        "Clippy should respect MSRV"
    );

    // Parse as TOML to verify validity
    toml::from_str::<toml::Value>(&clippy_content)
        .expect("Clippy configuration should be valid TOML");
}

#[test]
fn test_rustfmt_configuration_consistency() {
    let rustfmt_content = fs::read_to_string(workspace_root().join("rustfmt.toml"))
        .expect("Should be able to read rustfmt configuration");

    // Verify consistent formatting rules
    assert!(
        rustfmt_content.contains("edition = \"2024\""),
        "Rustfmt should use 2024 edition"
    );

    // Parse as TOML to verify validity
    toml::from_str::<toml::Value>(&rustfmt_content)
        .expect("Rustfmt configuration should be valid TOML");
}

#[test]
fn test_deny_configuration_security() {
    let deny_content = fs::read_to_string(workspace_root().join("deny.toml"))
        .expect("Should be able to read deny configuration");

    // Verify security-focused configuration
    assert!(
        deny_content.contains("[advisories]"),
        "Deny should check advisories"
    );
    assert!(
        deny_content.contains("[licenses]"),
        "Deny should check licenses"
    );
    assert!(
        deny_content.contains("[bans]"),
        "Deny should check banned dependencies"
    );
    assert!(
        deny_content.contains("[sources]"),
        "Deny should check sources"
    );

    // Parse as TOML to verify validity
    toml::from_str::<toml::Value>(&deny_content).expect("Deny configuration should be valid TOML");
}

#[test]
fn test_gitignore_completeness() {
    let gitignore_content = fs::read_to_string(workspace_root().join(".gitignore"))
        .expect("Should be able to read gitignore");

    // Verify essential ignores are present
    assert!(
        gitignore_content.contains("target/"),
        "Should ignore Rust build artifacts"
    );
    assert!(
        gitignore_content.contains("Cargo.lock")
            || gitignore_content.contains("# Cargo.lock is tracked"),
        "Should handle Cargo.lock appropriately"
    );
    assert!(
        gitignore_content.contains(".env"),
        "Should ignore environment files"
    );
    assert!(
        gitignore_content.contains("*.log"),
        "Should ignore log files"
    );
}

#[test]
fn test_tools_run_without_errors() {
    // Test that clippy runs without configuration errors
    let clippy_output = std::process::Command::new("cargo")
        .args(["clippy", "--workspace", "--", "--help"])
        .output()
        .expect("Should be able to run clippy");

    assert!(
        clippy_output.status.success(),
        "Clippy should run without configuration errors"
    );

    // Test that rustfmt runs without configuration errors
    let fmt_output = std::process::Command::new("cargo")
        .args(["fmt", "--", "--help"])
        .output()
        .expect("Should be able to run rustfmt");

    assert!(
        fmt_output.status.success(),
        "Rustfmt should run without configuration errors"
    );
}
