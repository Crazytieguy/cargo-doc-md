use std::process::Command;
use std::fs;
use std::path::PathBuf;

fn run_cargo_doc_md(args: &[&str]) -> Result<String, String> {
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("--").args(args);

    let output = cmd.output().expect("Failed to execute command");

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[test]
fn test_flag_validation_json_with_crates() {
    let result = run_cargo_doc_md(&["--json", "test.json", "tokio"]);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cannot use crate names with --json"));
}

#[test]
fn test_flag_validation_all_deps_with_crates() {
    let result = run_cargo_doc_md(&["--all-deps", "tokio"]);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cannot use --all-deps with specific crates"));
}

#[test]
fn test_json_validation_file_not_found() {
    let result = run_cargo_doc_md(&["--json", "nonexistent_file_12345.json"]);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("JSON file not found"));
}

#[test]
fn test_json_validation_path_is_directory() {
    // Create a temporary directory
    let temp_dir = std::env::temp_dir().join("cargo_doc_md_test_dir");
    fs::create_dir_all(&temp_dir).unwrap();

    let result = run_cargo_doc_md(&["--json", temp_dir.to_str().unwrap()]);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Path is not a file"));

    // Cleanup
    fs::remove_dir(&temp_dir).ok();
}

#[test]
fn test_migration_cleanup_behavior() {
    let output_dir = PathBuf::from("target/doc-md-test-migration");
    let deps_dir = output_dir.join("deps");

    fs::remove_dir_all(&output_dir).ok();
    fs::create_dir_all(&deps_dir).unwrap();
    fs::write(deps_dir.join("test.txt"), "test content").unwrap();

    // Run - may succeed or fail, but deps/ should be cleaned up
    let _ = run_cargo_doc_md(&["-o", output_dir.to_str().unwrap(), "--all-deps"]);

    // The key assertion: old deps/ directory should be removed
    assert!(!deps_dir.exists(), "Old deps/ directory should be cleaned up on any run");

    fs::remove_dir_all(&output_dir).ok();
}

#[test]
fn test_help_output() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("--").arg("--help");

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Combine both streams as help might be in either
    let combined = format!("{}{}", stdout, stderr);

    assert!(output.status.success());
    assert!(combined.contains("Generate markdown documentation") || combined.contains("Cargo subcommand"));
    assert!(combined.contains("--json"));
    assert!(combined.contains("--all-deps"));
    assert!(combined.contains("--output") || combined.contains("-o,"));
}

#[test]
fn test_output_directory_creation() {
    let output_dir = PathBuf::from("target/doc-md-test-creation");

    fs::remove_dir_all(&output_dir).ok();
    assert!(!output_dir.exists());

    // Run with --all-deps (may succeed or fail depending on dependencies)
    let _ = run_cargo_doc_md(&["-o", output_dir.to_str().unwrap(), "--all-deps"]);

    // Key assertion: output directory should be created
    assert!(output_dir.exists(), "Output directory should be created");

    fs::remove_dir_all(&output_dir).ok();
}
