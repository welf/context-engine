use std::fs;

use context_engine_tests::helpers::workspace_root;

#[test]
fn test_github_workflows_exist() {
    assert!(
        workspace_root().join(".github/workflows/ci.yml").exists(),
        "CI workflow must exist"
    );
    assert!(
        workspace_root()
            .join(".github/workflows/security.yml")
            .exists(),
        "Security workflow must exist"
    );
    assert!(
        workspace_root().join(".github/dependabot.yml").exists(),
        "Dependabot configuration must exist"
    );
}

#[test]
fn test_ci_workflow_completeness() {
    let ci_content = fs::read_to_string(workspace_root().join(".github/workflows/ci.yml"))
        .expect("Should be able to read CI workflow");

    // Verify essential CI steps are present
    assert!(ci_content.contains("cargo test"), "CI must run tests");
    assert!(ci_content.contains("cargo clippy"), "CI must run clippy");
    assert!(ci_content.contains("cargo fmt"), "CI must check formatting");
    assert!(
        ci_content.contains("cargo audit"),
        "CI must run security audit"
    );
    assert!(ci_content.contains("cargo deny"), "CI must check licenses");

    // Verify multiple Rust versions are tested
    // assert!(
    //     ci_content.contains("1.87.0"),
    //     "CI must test minimum Rust version"
    // );
    assert!(ci_content.contains("nightly"), "CI must test nightly Rust");

    // Verify cross-platform testing
    assert!(ci_content.contains("ubuntu"), "CI must test on Linux");
    assert!(ci_content.contains("windows"), "CI must test on Windows");
    assert!(ci_content.contains("macos"), "CI must test on macOS");
}

#[test]
fn test_security_workflow_completeness() {
    let security_content =
        fs::read_to_string(workspace_root().join(".github/workflows/security.yml"))
            .expect("Should be able to read security workflow");

    // Verify security scanning steps
    assert!(
        security_content.contains("cargo audit"),
        "Security workflow must audit dependencies"
    );
    assert!(
        security_content.contains("cargo deny"),
        "Security workflow must check licenses"
    );

    // Verify it runs on dependency changes
    assert!(
        security_content.contains("Cargo.toml"),
        "Security workflow must trigger on Cargo.toml changes"
    );
    assert!(
        security_content.contains("Cargo.lock"),
        "Security workflow must trigger on Cargo.lock changes"
    );
}

#[test]
fn test_dependabot_configuration() {
    let dependabot_content = fs::read_to_string(workspace_root().join(".github/dependabot.yml"))
        .expect("Should be able to read dependabot configuration");

    // Verify Rust ecosystem is configured
    assert!(
        dependabot_content.contains("cargo"),
        "Dependabot must monitor Cargo dependencies"
    );
    assert!(
        dependabot_content.contains("daily"),
        "Dependabot should check daily for security updates"
    );
}

#[test]
fn test_workflow_yaml_validity() {
    // Test that workflow files are valid YAML
    let ci_content = fs::read_to_string(workspace_root().join(".github/workflows/ci.yml"))
        .expect("Should be able to read CI workflow");

    serde_yaml::from_str::<serde_yaml::Value>(&ci_content)
        .expect("CI workflow should be valid YAML");

    let security_content =
        fs::read_to_string(workspace_root().join(".github/workflows/security.yml"))
            .expect("Should be able to read security workflow");

    serde_yaml::from_str::<serde_yaml::Value>(&security_content)
        .expect("Security workflow should be valid YAML");
}
