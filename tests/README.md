# Snapshot Tests

This directory contains snapshot tests for the cargo-doc-md tool. Snapshot testing ensures that the markdown output remains consistent across changes.

## Structure

- `fixtures/` - Test data
  - `test_crate/` - A comprehensive test crate covering all Rust documentation features
  - `test_crate.json` - Pre-generated rustdoc JSON for the test crate
- `snapshot_tests.rs` - Snapshot test suite
- `snapshots/` - Stored snapshots of expected output (auto-generated)

## Running Tests

```bash
# Run all snapshot tests
cargo test --test snapshot_tests

# Run all tests
cargo test
```

## Updating Snapshots

When you make intentional changes to the output format:

```bash
# Run tests to see differences
cargo test --test snapshot_tests

# Review and accept changes
cargo insta review

# Or accept all changes
cargo insta accept
```

## What's Tested

The snapshot tests cover:

- **Structs**: Plain, tuple, unit, with generics, with methods
- **Enums**: Simple and complex variants
- **Functions**: Various signatures including generic and async functions
- **Traits**: Trait definitions and implementations
- **Modules**: Nested module hierarchies
- **Constants and Type Aliases**
- **Multi-file output structure**
- **Index generation**
- **Legacy single-file output**

## How Snapshot Tests Help with PRs

Snapshot tests make it easy to see exactly how output has changed:

1. When tests fail, `insta` shows a diff of the changes
2. Reviewers can see the exact markdown output differences in the PR
3. Snapshots are committed to git, so changes are visible in diffs
4. This makes it easy to verify that changes are intentional and correct

## Regenerating Test Fixtures

If you need to regenerate the test fixture JSON:

```bash
cd tests/fixtures/test_crate
cargo +nightly rustdoc --lib -- --output-format=json -Z unstable-options
cp target/doc/test_crate.json ../test_crate.json
```
