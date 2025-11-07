# cargo-doc-md

> **🤖 AI-Generated Project**: Created entirely by Claude (Anthropic AI). See [ATTRIBUTION.md](ATTRIBUTION.md).

A Cargo subcommand that generates markdown documentation for Rust crates and their dependencies.

## Installation

**Requires Rust nightly** (uses unstable rustdoc features):
```bash
rustup install nightly
cargo install cargo-doc-md
```

## Usage

```bash
# Document current crate + all dependencies (like cargo doc)
cargo doc-md

# Document current crate only (no dependencies)
cargo doc-md --no-deps

# Document specific packages
cargo doc-md -p tokio -p serde -p axum

# Document all workspace members + dependencies
cargo doc-md --workspace

# Custom output directory
cargo doc-md -o docs/

# Convert existing rustdoc JSON
cargo doc-md --json target/doc/my_crate.json
```

### Output Structure

```
target/doc-md/
  index.md                    # Master index
  your_crate/
    index.md                  # Crate overview
    module1.md                # One file per module
    module2.md
    sub/
      nested_module.md
  tokio/
    index.md
    io.md
    net.md
  serde/
    index.md
    ...
```

### Use Cases

- Provide API documentation to LLMs as context
- Read documentation in your terminal or editor
- Generate offline documentation for your entire dependency tree
- Navigate large codebases with multi-file output

## Features

- Multi-file output with one markdown file per module
- Master index listing all documented crates
- Breadcrumb navigation showing module hierarchy
- Module summaries with item counts
- Complete type signatures for all items
- Automatic dependency discovery and documentation
- Handles multiple versions of the same dependency
- Gracefully skips dependencies that fail to build

## Options

```
cargo doc-md [OPTIONS]

Options:
  -p, --package <PACKAGE>   Package(s) to document with their dependencies (can be repeated)
  -o, --output <DIR>        Output directory [default: target/doc-md]
      --workspace           Document all workspace members
      --no-deps             Don't document dependencies
      --include-private     Include private items
      --json <FILE>         Convert existing rustdoc JSON file
  -h, --help                Show help
```

Run `cargo doc-md --help` for detailed information.

## Upgrading from 0.7.x

**Breaking changes** in 0.8.0:
- Directory structure flattened: `deps/crate/` → `crate/`
- CLI changed: `--deps tokio,serde` → `-p tokio -p serde`
- JSON conversion: now requires `--json` flag
- Workspace support: use `--workspace` flag to document all members

Tool auto-migrates old structure on first run. Update your scripts accordingly.

## Development

This project uses snapshot testing to ensure output quality and consistency. Run tests with:

```bash
cargo test
```

When making changes to the output format, review snapshot changes with:

```bash
cargo insta review
```

See [tests/README.md](tests/README.md) for more information about the test suite.

## License

MIT or Apache-2.0
