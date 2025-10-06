use cargo_doc_md::{converter, parser, ConversionOptions};
use std::path::Path;

#[test]
fn test_multifile_output_structure() {
    let json_path = Path::new("tests/fixtures/test_crate.json");
    let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
    let output = converter::convert_to_markdown_multifile(&crate_data, false)
        .expect("Failed to convert to markdown");

    let mut file_paths: Vec<_> = output.files.keys().collect();
    file_paths.sort();

    insta::assert_debug_snapshot!("multifile_structure", file_paths);
    assert_eq!(output.crate_name, "test_crate");
}

#[test]
fn test_index_file() {
    let json_path = Path::new("tests/fixtures/test_crate.json");
    let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
    let output = converter::convert_to_markdown_multifile(&crate_data, false)
        .expect("Failed to convert to markdown");

    let index_content = output.files.get("index.md").expect("index.md not found");
    insta::assert_snapshot!("index_md", index_content);
}

#[test]
fn test_lib_module() {
    let json_path = Path::new("tests/fixtures/test_crate.json");
    let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
    let output = converter::convert_to_markdown_multifile(&crate_data, false)
        .expect("Failed to convert to markdown");

    if let Some(lib_content) = output.files.get("test_crate.md") {
        insta::assert_snapshot!("lib_module", lib_content);
    }
}

#[test]
fn test_types_module() {
    let json_path = Path::new("tests/fixtures/test_crate.json");
    let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
    let output = converter::convert_to_markdown_multifile(&crate_data, false)
        .expect("Failed to convert to markdown");

    if let Some(types_content) = output.files.get("types.md") {
        insta::assert_snapshot!("types_module", types_content);
    }
}

#[test]
fn test_functions_module() {
    let json_path = Path::new("tests/fixtures/test_crate.json");
    let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
    let output = converter::convert_to_markdown_multifile(&crate_data, false)
        .expect("Failed to convert to markdown");

    if let Some(functions_content) = output.files.get("functions.md") {
        insta::assert_snapshot!("functions_module", functions_content);
    }
}

#[test]
fn test_nested_modules() {
    let json_path = Path::new("tests/fixtures/test_crate.json");
    let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
    let output = converter::convert_to_markdown_multifile(&crate_data, false)
        .expect("Failed to convert to markdown");

    if let Some(nested_content) = output.files.get("nested.md") {
        insta::assert_snapshot!("nested_module", nested_content);
    }

    if let Some(inner_content) = output.files.get("nested/inner.md") {
        insta::assert_snapshot!("nested_inner_module", inner_content);
    }

    if let Some(deep_content) = output.files.get("nested/inner/deep.md") {
        insta::assert_snapshot!("nested_deep_module", deep_content);
    }
}

#[test]
fn test_struct_formatting() {
    let json_path = Path::new("tests/fixtures/test_crate.json");
    let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
    let output = converter::convert_to_markdown_multifile(&crate_data, false)
        .expect("Failed to convert to markdown");

    let lib_content = output.files.get("test_crate.md").expect("lib module not found");

    assert!(lib_content.contains("## test_crate::UnitStruct"));
    assert!(lib_content.contains("**Unit Struct**"));

    assert!(lib_content.contains("## test_crate::TupleStruct"));
    assert!(lib_content.contains("**Tuple Struct**"));

    assert!(lib_content.contains("## test_crate::PlainStruct"));
    assert!(lib_content.contains("**Fields:**"));
}

#[test]
fn test_generic_struct_formatting() {
    let json_path = Path::new("tests/fixtures/test_crate.json");
    let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
    let output = converter::convert_to_markdown_multifile(&crate_data, false)
        .expect("Failed to convert to markdown");

    let lib_content = output.files.get("test_crate.md").expect("lib module not found");

    assert!(lib_content.contains("## test_crate::GenericStruct"));
    assert!(lib_content.contains("**Generic Parameters:**"));
}

#[test]
fn test_enum_formatting() {
    let json_path = Path::new("tests/fixtures/test_crate.json");
    let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
    let output = converter::convert_to_markdown_multifile(&crate_data, false)
        .expect("Failed to convert to markdown");

    let lib_content = output.files.get("test_crate.md").expect("lib module not found");

    assert!(lib_content.contains("## test_crate::SimpleEnum"));
    assert!(lib_content.contains("**Variants:**"));

    assert!(lib_content.contains("## test_crate::ComplexEnum"));
}

#[test]
fn test_function_formatting() {
    let json_path = Path::new("tests/fixtures/test_crate.json");
    let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
    let output = converter::convert_to_markdown_multifile(&crate_data, false)
        .expect("Failed to convert to markdown");

    let functions_content = output.files.get("functions.md").expect("functions module not found");

    assert!(functions_content.contains("fn add"));
    assert!(functions_content.contains("fn multiply"));
}

#[test]
fn test_trait_formatting() {
    let json_path = Path::new("tests/fixtures/test_crate.json");
    let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
    let output = converter::convert_to_markdown_multifile(&crate_data, false)
        .expect("Failed to convert to markdown");

    let lib_content = output.files.get("test_crate.md").expect("lib module not found");

    assert!(lib_content.contains("## test_crate::MyTrait"));
    assert!(lib_content.contains("**Type:** Trait"));
}

#[test]
fn test_methods_and_impls() {
    let json_path = Path::new("tests/fixtures/test_crate.json");
    let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
    let output = converter::convert_to_markdown_multifile(&crate_data, false)
        .expect("Failed to convert to markdown");

    let lib_content = output.files.get("test_crate.md").expect("lib module not found");

    assert!(lib_content.contains("**Methods:**"));
}

#[test]
fn test_constants_and_type_aliases() {
    let json_path = Path::new("tests/fixtures/test_crate.json");
    let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
    let output = converter::convert_to_markdown_multifile(&crate_data, false)
        .expect("Failed to convert to markdown");

    let lib_content = output.files.get("test_crate.md").expect("lib module not found");

    assert!(lib_content.contains("MAX_SIZE"));
    assert!(lib_content.contains("Result"));
}

#[test]
fn test_include_private() {
    let json_path = Path::new("tests/fixtures/test_crate.json");
    let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");

    let output_public = converter::convert_to_markdown_multifile(&crate_data, false)
        .expect("Failed to convert to markdown");
    let output_private = converter::convert_to_markdown_multifile(&crate_data, true)
        .expect("Failed to convert to markdown");

    let public_count = output_public.files.len();
    let private_count = output_private.files.len();

    assert!(private_count >= public_count,
        "Private output should have at least as many files as public output");
}

#[test]
fn test_conversion_end_to_end() {
    let output_dir = std::env::temp_dir().join("cargo_doc_md_test");
    let _ = std::fs::remove_dir_all(&output_dir);

    let json_path = Path::new("tests/fixtures/test_crate.json");
    let options = ConversionOptions {
        input_path: json_path,
        output_dir: &output_dir,
        include_private: false,
    };

    cargo_doc_md::convert_json_file(&options).expect("Conversion failed");

    assert!(output_dir.join("test_crate").exists());
    assert!(output_dir.join("test_crate/index.md").exists());

    std::fs::remove_dir_all(&output_dir).ok();
}

#[test]
fn test_legacy_single_file_output() {
    let json_path = Path::new("tests/fixtures/test_crate.json");
    let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
    let markdown = converter::convert_to_markdown(&crate_data, false)
        .expect("Failed to convert to markdown");

    assert!(markdown.contains("# test_crate"));
    assert!(markdown.contains("## Table of Contents"));
    insta::assert_snapshot!("legacy_single_file", markdown);
}
