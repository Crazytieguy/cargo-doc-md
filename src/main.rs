use anyhow::{Context, Result, bail};
use cargo_doc_md::ConversionOptions;
use clap::Parser;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(name = "cargo-doc-md")]
#[command(
    about = "Generate markdown documentation for Rust crates and dependencies",
    long_about = "Cargo subcommand to generate markdown documentation for Rust crates and their dependencies.\n\n\
                  Default behavior: Documents current crate + all dependencies with multi-file output.\n\
                  Creates a master index and organizes modules into separate files for easy navigation.\n\n\
                  Usage:\n  \
                  cargo doc-md                  # Document current crate + all dependencies\n  \
                  cargo doc-md tokio serde      # Document specific crates\n  \
                  cargo doc-md --all-deps       # Document only dependencies\n  \
                  cargo doc-md --json file.json # Convert existing rustdoc JSON"
)]
struct Cli {
    #[arg(help = "Specific crate(s) to document (omit for current crate + all deps)")]
    crates: Vec<String>,

    #[arg(
        short,
        long,
        default_value = "target/doc-md",
        help = "Output directory [default: target/doc-md]\n\
                Creates: target/doc-md/index.md (master index), target/doc-md/crate_name/*.md (modules)"
    )]
    output: PathBuf,

    #[arg(long, help = "Include private items in documentation")]
    include_private: bool,

    #[arg(long, help = "Convert existing rustdoc JSON file")]
    json: Option<PathBuf>,

    #[arg(
        long,
        help = "Document only all direct dependencies (excludes current crate)"
    )]
    all_deps: bool,
}

fn main() -> Result<()> {
    // When invoked as `cargo doc-md`, cargo passes an extra "doc-md" argument
    // Skip it if present to support both `cargo doc-md` and `cargo-doc-md` invocations
    let args = std::env::args()
        .enumerate()
        .filter(|(i, arg)| !(*i == 1 && arg == "doc-md"))
        .map(|(_, arg)| arg);

    let cli = Cli::parse_from(args);

    // Validate flag combinations
    if cli.json.is_some() && !cli.crates.is_empty() {
        bail!("Cannot specify crate names with --json flag");
    }
    if cli.all_deps && !cli.crates.is_empty() {
        bail!("Cannot use --all-deps with specific crate names");
    }

    // Verify nightly toolchain is available (unless only using --json mode)
    if cli.json.is_none() {
        check_nightly_toolchain()?;
    }

    // Clean up old directory structure (migration from v0.7.x)
    cleanup_old_structure(&cli.output)?;

    // Explicit JSON file - just convert that file
    if let Some(json_path) = cli.json.as_ref() {
        if !json_path.exists() {
            bail!("JSON file not found: {}", json_path.display());
        }
        if !json_path.is_file() {
            bail!("Path is not a file: {}", json_path.display());
        }
        let options = ConversionOptions {
            input_path: json_path,
            output_dir: &cli.output,
            include_private: cli.include_private,
        };

        cargo_doc_md::convert_json_file(&options)?;

        // Determine the crate name from the JSON file path
        let crate_name = json_path
            .file_stem()
            .and_then(|s| s.to_str())
            .context("Invalid JSON filename - could not extract crate name")?;
        println!(
            "✓ Conversion complete! Output written to: {}/{}/index.md",
            cli.output.display(),
            crate_name
        );
        return Ok(());
    }

    // Specific crates requested
    if !cli.crates.is_empty() {
        document_specific_crates(&cli)?;
        return Ok(());
    }

    // Dependency-only mode
    if cli.all_deps {
        document_dependencies(&cli)?;
        return Ok(());
    }

    // Default: document current crate + all dependencies (like cargo doc)
    println!("📚 Documenting current crate and all dependencies...\n");

    let current_crate = document_current_crate(&cli)?;
    println!();
    let documented_deps = document_all_dependencies(&cli)?;

    // Generate master index
    generate_master_index(&cli.output, current_crate.as_deref(), &documented_deps)?;

    Ok(())
}

struct Dependency {
    name: String,
    version: String,
}

fn check_nightly_toolchain() -> Result<()> {
    let output = Command::new("cargo")
        .args(["+nightly", "--version"])
        .output()
        .context("Failed to run cargo +nightly")?;

    if !output.status.success() {
        bail!(
            "Nightly toolchain not installed or not available.\n\
             This tool requires Rust nightly for unstable rustdoc features.\n\
             Install with: rustup install nightly"
        );
    }

    Ok(())
}

fn cleanup_old_structure(output_dir: &Path) -> Result<()> {
    use std::fs;

    let old_deps_dir = output_dir.join("deps");
    if old_deps_dir.exists() && old_deps_dir.is_dir() {
        println!("⚠  Cleaning up old directory structure ({})", old_deps_dir.display());
        if let Err(e) = fs::remove_dir_all(&old_deps_dir) {
            println!("⚠  Could not remove old deps directory: {}", e);
            println!("   You may need to manually delete: {}", old_deps_dir.display());
        } else {
            println!("✓ Migrated to new flat structure\n");
        }
    }
    Ok(())
}

fn document_specific_crates(cli: &Cli) -> Result<()> {
    println!("📦 Documenting {} specific crate(s)...", cli.crates.len());

    let mut successful = Vec::new();
    let mut failed = Vec::new();

    for crate_name in &cli.crates {
        println!("\n🔨 Generating docs for '{}'...", crate_name);

        let dep = Dependency {
            name: crate_name.clone(),
            version: String::new(),
        };

        match document_single_dependency(&dep, &cli.output, cli.include_private) {
            Ok(()) => {
                successful.push(crate_name.clone());
                println!("  ✓ {} → {}/{}/index.md", crate_name, cli.output.display(), crate_name);
            }
            Err(e) => {
                failed.push(crate_name.clone());
                println!("  ✗ Failed to document '{}': {}", crate_name, e);
            }
        }
    }

    println!("\n📊 Summary:");
    println!("  ✓ Successful: {}", successful.len());
    if !failed.is_empty() {
        println!("  ✗ Failed: {} ({})", failed.len(), failed.join(", "));
    }

    if !successful.is_empty() {
        generate_master_index(&cli.output, None, &successful)?;
    }

    Ok(())
}

fn document_current_crate(cli: &Cli) -> Result<Option<String>> {
    println!("🔨 Generating rustdoc JSON for current crate...");

    // Run cargo rustdoc to generate JSON
    let output = Command::new("cargo")
        .args([
            "+nightly",
            "rustdoc",
            "--lib",
            "--",
            "--output-format=json",
            "-Z",
            "unstable-options",
        ])
        .output()
        .context("Failed to run cargo rustdoc")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        if stderr.contains("no library targets found") {
            println!("⚠ No library target found in current crate, skipping current crate documentation");
            return Ok(None);
        }

        bail!("cargo rustdoc failed:\n{}", stderr);
    }

    // Get the crate name from cargo metadata
    let metadata_output = Command::new("cargo")
        .args(["metadata", "--format-version=1", "--no-deps"])
        .output()
        .context("Failed to run cargo metadata")?;

    if !metadata_output.status.success() {
        bail!(
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&metadata_output.stderr)
        );
    }

    let metadata: serde_json::Value = serde_json::from_slice(&metadata_output.stdout)
        .context("Failed to parse cargo metadata")?;

    let packages = metadata["packages"]
        .as_array()
        .context("Missing 'packages' in metadata")?;

    // Get the root package name using resolve.root
    let root_id = metadata["resolve"]["root"]
        .as_str()
        .context("Missing 'resolve.root' in metadata - is this a workspace without a default package?")?;

    let crate_name = packages
        .iter()
        .find(|p| p["id"].as_str() == Some(root_id))
        .and_then(|p| p["name"].as_str())
        .context("Root package not found in packages list")?
        .to_string();

    // Find the generated JSON file
    let json_path =
        PathBuf::from("target/doc").join(format!("{}.json", crate_name.replace("-", "_")));

    if !json_path.exists() {
        bail!("Generated JSON file not found at {}", json_path.display());
    }

    println!("✓ JSON generated successfully");
    println!("🔄 Converting to markdown...");

    // Convert to markdown
    let options = ConversionOptions {
        input_path: &json_path,
        output_dir: &cli.output,
        include_private: cli.include_private,
    };

    cargo_doc_md::convert_json_file(&options)?;

    println!(
        "✓ Current crate documented: {}/{}/index.md",
        cli.output.display(),
        crate_name
    );

    Ok(Some(crate_name))
}

fn document_dependencies_internal(
    deps_to_document: &[Dependency],
    output_dir: &Path,
    include_private: bool,
) -> (Vec<String>, Vec<String>) {
    let mut successful = Vec::new();
    let mut failed = Vec::new();

    for dep in deps_to_document {
        match document_single_dependency(dep, output_dir, include_private) {
            Ok(()) => {
                successful.push(dep.name.clone());
                println!("  ✓ {} → {}/{}/index.md", dep.name, output_dir.display(), dep.name);
            }
            Err(e) => {
                failed.push(dep.name.clone());
                println!("  ✗ {} - {}", dep.name, e);
            }
        }
    }

    (successful, failed)
}

fn print_documentation_summary(successful: &[String], failed: &[String]) {
    println!("\n📊 Summary:");
    println!("  ✓ Successful: {}", successful.len());
    if !failed.is_empty() {
        println!("  ✗ Failed: {} ({})", failed.len(), failed.join(", "));
    }
}

fn document_all_dependencies(cli: &Cli) -> Result<Vec<String>> {
    let deps_to_document = get_all_dependencies()?;

    if deps_to_document.is_empty() {
        println!("No dependencies found");
        return Ok(Vec::new());
    }

    println!("📦 Documenting {} dependencies...", deps_to_document.len());

    let (successful, failed) = document_dependencies_internal(
        &deps_to_document,
        &cli.output,
        cli.include_private,
    );

    print_documentation_summary(&successful, &failed);

    Ok(successful)
}

fn document_dependencies(cli: &Cli) -> Result<()> {
    let deps_to_document = get_all_dependencies()?;

    if deps_to_document.is_empty() {
        bail!("No dependencies found to document");
    }

    println!("📦 Documenting {} dependencies...", deps_to_document.len());

    let (successful, failed) = document_dependencies_internal(
        &deps_to_document,
        &cli.output,
        cli.include_private,
    );

    print_documentation_summary(&successful, &failed);

    if !successful.is_empty() {
        generate_master_index(&cli.output, None, &successful)?;
    }

    Ok(())
}

fn get_all_dependencies() -> Result<Vec<Dependency>> {
    // Use cargo metadata to get all direct dependencies
    let output = Command::new("cargo")
        .args(["metadata", "--format-version=1"])
        .output()
        .context("Failed to run 'cargo metadata'")?;

    if !output.status.success() {
        bail!(
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).context("Failed to parse cargo metadata")?;

    let packages = metadata["packages"]
        .as_array()
        .context("Missing 'packages' in metadata")?;

    // Get the root package
    let resolve = &metadata["resolve"];
    let root = resolve["root"]
        .as_str()
        .context("Missing 'root' in metadata")?;

    // Find dependencies of the root package
    let nodes = resolve["nodes"]
        .as_array()
        .context("Missing 'nodes' in resolve")?;

    let root_node = nodes
        .iter()
        .find(|n| n["id"].as_str() == Some(root))
        .context("Root package not found in nodes")?;

    let dep_ids: Vec<String> = root_node["dependencies"]
        .as_array()
        .context("Missing 'dependencies' in root node")?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    // Map package IDs to package names and versions
    let mut deps = Vec::new();
    for dep_id in dep_ids {
        if let Some(pkg) = packages.iter().find(|p| p["id"].as_str() == Some(&dep_id)) {
            if let (Some(name), Some(version)) = (pkg["name"].as_str(), pkg["version"].as_str()) {
                deps.push(Dependency {
                    name: name.to_string(),
                    version: version.to_string(),
                });
            }
        }
    }

    deps.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(deps)
}

fn document_single_dependency(
    dep: &Dependency,
    output_base: &Path,
    include_private: bool,
) -> Result<()> {
    // Build the package specification
    // If we have a version, use name@version to disambiguate multiple versions
    let package_spec = if dep.version.is_empty() {
        dep.name.clone()
    } else {
        format!("{}@{}", dep.name, dep.version)
    };

    // Generate rustdoc JSON for the dependency
    let output = Command::new("cargo")
        .args([
            "+nightly",
            "rustdoc",
            "-p",
            &package_spec,
            "--lib",
            "--",
            "--output-format=json",
            "-Z",
            "unstable-options",
        ])
        .output()
        .context("Failed to run cargo rustdoc")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Check for known non-error cases
        if stderr.contains("no library targets found") {
            bail!("no library target found (binary-only crate)");
        }
        // Show abbreviated error to avoid clutter
        let error_summary = stderr
            .lines()
            .filter(|line| line.contains("error") || line.contains("failed"))
            .take(3)
            .collect::<Vec<_>>()
            .join("; ");
        if !error_summary.is_empty() {
            bail!("build failed: {}", error_summary);
        }
        bail!("cargo rustdoc failed (exit code: {})", output.status);
    }

    // Find the generated JSON file
    let json_path =
        PathBuf::from("target/doc").join(format!("{}.json", dep.name.replace("-", "_")));

    if !json_path.exists() {
        bail!("Generated JSON file not found at {}", json_path.display());
    }

    // Convert to markdown directly in output directory
    // The converter will create a subdirectory with the crate name
    let options = ConversionOptions {
        input_path: &json_path,
        output_dir: output_base,
        include_private,
    };

    cargo_doc_md::convert_json_file(&options)?;

    Ok(())
}

fn generate_master_index(
    output_dir: &Path,
    current_crate: Option<&str>,
    dependencies: &[String],
) -> Result<()> {
    use std::fs;

    let mut content = String::new();

    content.push_str("# Documentation Index\n\n");
    content.push_str("Generated markdown documentation for this project.\n\n");

    // Current crate section
    if let Some(crate_name) = current_crate {
        content.push_str("## Current Crate\n\n");
        content.push_str(&format!(
            "- [`{}`]({}index.md)\n\n",
            crate_name,
            crate_name.to_string() + "/"
        ));
    }

    // Dependencies section
    if !dependencies.is_empty() {
        content.push_str(&format!("## Dependencies ({})\n\n", dependencies.len()));

        for dep in dependencies {
            let dep_path = format!("{}/index.md", dep);
            content.push_str(&format!("- [`{}`]({})\n", dep, dep_path));
        }
        content.push('\n');
    }

    content.push_str("---\n\n");
    content.push_str(
        "Generated with [cargo-doc-md](https://github.com/Crazytieguy/cargo-doc-md)\n",
    );

    let index_path = output_dir.join("index.md");
    fs::write(&index_path, content)
        .with_context(|| format!("Failed to write master index: {}", index_path.display()))?;

    println!("\n✓ Master index: {}", index_path.display());

    Ok(())
}
