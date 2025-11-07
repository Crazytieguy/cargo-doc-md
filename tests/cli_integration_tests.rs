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
fn test_flag_validation_json_with_package() {
    let result = run_cargo_doc_md(&["--json", "test.json", "-p", "tokio"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    // Clap's conflict error message
    assert!(err.contains("cannot be used with") || err.contains("conflicts with"));
}

#[test]
fn test_flag_validation_workspace_with_package() {
    let result = run_cargo_doc_md(&["--workspace", "-p", "tokio"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    // Clap's conflict error message
    assert!(err.contains("cannot be used with") || err.contains("conflicts with"));
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
    let _ = run_cargo_doc_md(&["-o", output_dir.to_str().unwrap()]);

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
    assert!(combined.contains("--workspace"));
    assert!(combined.contains("--package") || combined.contains("-p,"));
    assert!(combined.contains("--output") || combined.contains("-o,"));
}

#[test]
fn test_output_directory_creation() {
    let output_dir = PathBuf::from("target/doc-md-test-creation");

    fs::remove_dir_all(&output_dir).ok();
    assert!(!output_dir.exists());

    // Run with --no-deps (will document current crate only, should succeed)
    let _ = run_cargo_doc_md(&["-o", output_dir.to_str().unwrap(), "--no-deps"]);

    // Key assertion: output directory should be created
    assert!(output_dir.exists(), "Output directory should be created");

    fs::remove_dir_all(&output_dir).ok();
}

#[test]
fn test_package_flag_single() {
    // Test that -p flag is accepted (gracefully fails if package doesn't exist)
    let result = run_cargo_doc_md(&["-p", "nonexistent-crate-12345"]);
    // May succeed (exit 0) with failure message or error out
    if result.is_err() {
        let err = result.unwrap_err();
        // Should NOT contain flag validation errors
        assert!(!err.contains("Cannot use"));
    } else {
        // If it succeeded, output should mention the failure
        let output = result.unwrap();
        assert!(output.contains("Failed") || output.contains("failed"));
    }
}

#[test]
fn test_package_flag_multiple() {
    // Test that multiple -p flags are accepted
    let result = run_cargo_doc_md(&["-p", "nonexistent-crate-1", "-p", "nonexistent-crate-2"]);
    // May succeed (exit 0) with failure messages or error out
    if result.is_err() {
        let err = result.unwrap_err();
        // Should NOT contain flag validation errors
        assert!(!err.contains("Cannot use"));
    } else {
        // If it succeeded, should report failures
        let output = result.unwrap();
        assert!(output.contains("Failed") || output.contains("Summary"));
    }
}

#[test]
fn test_workspace_flag_in_single_crate() {
    // Test --workspace in a single-crate project (this project)
    let result = run_cargo_doc_md(&["--workspace", "--no-deps"]);
    // Should succeed - this is a single-member workspace
    // (or fail gracefully if not in workspace)
    if result.is_err() {
        let err = result.unwrap_err();
        // If it errors, should be about workspace detection, not flag validation
        assert!(
            err.contains("workspace") || err.contains("Workspace"),
            "Error should be about workspace detection: {}",
            err
        );
    }
}

#[test]
fn test_workspace_with_no_deps() {
    // Test that --workspace --no-deps combination is valid
    let result = run_cargo_doc_md(&["--workspace", "--no-deps"]);
    if result.is_err() {
        let err = result.unwrap_err();
        // Should NOT be a flag validation error
        assert!(!err.contains("Cannot use --workspace with"));
    }
}

#[test]
fn test_output_directory_validation_is_file() {
    use std::io::Write;

    let temp_file = std::env::temp_dir().join("cargo_doc_md_test_file.txt");
    let mut file = fs::File::create(&temp_file).unwrap();
    file.write_all(b"test").unwrap();
    drop(file);

    let result = run_cargo_doc_md(&["-o", temp_file.to_str().unwrap()]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("is a file, not a directory"), "Error should mention file vs directory: {}", err);

    fs::remove_file(&temp_file).ok();
}

#[test]
fn test_output_directory_validation_parent_missing() {
    let nonexistent_parent = PathBuf::from("/nonexistent_parent_dir_12345/output");

    let result = run_cargo_doc_md(&["-o", nonexistent_parent.to_str().unwrap()]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Parent directory does not exist"), "Error should mention parent directory: {}", err);
}
