# Changelog

## [0.8.0] - Unreleased

### Changed (Breaking)
- **Flattened directory structure**: Dependencies now at `target/doc-md/crate/` instead of `target/doc-md/deps/crate/`
- **CLI redesign**: Positional args now accept crate names instead of JSON paths
  - Old: `cargo doc-md --deps tokio,serde`
  - New: `cargo doc-md tokio serde`
- **JSON conversion**: Now requires explicit `--json` flag
  - Old: `cargo doc-md file.json`
  - New: `cargo doc-md --json file.json`

### Added
- Automatic migration from old `deps/` structure on first run
- Nightly toolchain validation with helpful error messages
- Flag conflict validation (e.g., `--json` + crate names)
- Better error messages with actionable suggestions
- CLI integration tests

### Fixed
- Root package lookup now uses `resolve.root` (correct for workspaces)
- Stderr shown on build failures (was suppressed)
- Master index now generated in all code paths

### Migration from 0.7.x
Tool auto-migrates on first run. Update scripts:
- Replace `--deps crate1,crate2` with `crate1 crate2`
- Replace `cargo doc-md file.json` with `cargo doc-md --json file.json`
- Update file paths: `deps/crate/` → `crate/`
