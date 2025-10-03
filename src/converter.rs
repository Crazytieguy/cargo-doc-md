//! Markdown converter for rustdoc JSON data.

use anyhow::Result;
use rustdoc_types::{Crate, Item, ItemEnum, Visibility, Id};
use std::collections::HashMap;

/// Represents the multi-file markdown output
pub struct MarkdownOutput {
    /// Crate name
    pub crate_name: String,
    /// Map of relative file path -> content
    pub files: HashMap<String, String>,
}

/// Convert a rustdoc Crate to multi-file markdown format.
pub fn convert_to_markdown_multifile(crate_data: &Crate, include_private: bool) -> Result<MarkdownOutput> {
    let root_item = crate_data.index.get(&crate_data.root)
        .ok_or_else(|| anyhow::anyhow!("Root item not found in index"))?;

    let crate_name = root_item.name.as_deref().unwrap_or("unknown");

    // Build a map of item_id -> full_path using the paths data
    let item_paths = build_path_map(crate_data);

    // Group items by module
    let modules = group_by_module(crate_data, &item_paths, include_private);

    let mut files = HashMap::new();

    // Generate index.md with crate overview and module list
    let index_content = generate_crate_index(crate_name, root_item, &modules);
    files.insert("index.md".to_string(), index_content);

    // Generate one file per module
    for (module_name, items) in &modules {
        let module_filename = module_name
            .strip_prefix(&format!("{}::", crate_name))
            .unwrap_or(module_name)
            .replace("::", "/");

        let file_path = format!("{}.md", module_filename);
        let module_content = generate_module_file(module_name, items, crate_data, &item_paths, crate_name);
        files.insert(file_path, module_content);
    }

    Ok(MarkdownOutput {
        crate_name: crate_name.to_string(),
        files,
    })
}

/// Convert a rustdoc Crate to markdown format (legacy single-file).
pub fn convert_to_markdown(crate_data: &Crate, include_private: bool) -> Result<String> {
    let mut output = String::new();

    let root_item = crate_data.index.get(&crate_data.root)
        .ok_or_else(|| anyhow::anyhow!("Root item not found in index"))?;

    let crate_name = root_item.name.as_deref().unwrap_or("unknown");
    output.push_str(&format!("# {}\n\n", crate_name));

    if let Some(docs) = &root_item.docs {
        output.push_str(&format!("{}\n\n", docs));
    }

    // Build a map of item_id -> full_path using the paths data
    let item_paths = build_path_map(crate_data);

    // Group items by module
    let modules = group_by_module(crate_data, &item_paths, include_private);

    // Generate hierarchical ToC
    output.push_str("## Table of Contents\n\n");
    output.push_str(&generate_toc(&modules, crate_name));
    output.push_str("\n\n---\n\n");

    // Generate content organized by module
    output.push_str(&generate_content(&modules, crate_data, &item_paths));

    Ok(output)
}

fn build_path_map(crate_data: &Crate) -> HashMap<Id, Vec<String>> {
    crate_data.paths.iter()
        .map(|(id, summary)| {
            (*id, summary.path.clone())
        })
        .collect()
}

fn group_by_module(
    crate_data: &Crate,
    item_paths: &HashMap<Id, Vec<String>>,
    include_private: bool,
) -> HashMap<String, Vec<(Id, Item)>> {
    let mut modules: HashMap<String, Vec<(Id, Item)>> = HashMap::new();

    for (id, item) in &crate_data.index {
        if id == &crate_data.root {
            continue;
        }

        if !include_private && !is_public(item) {
            continue;
        }

        // Skip if we can't format this item type
        if !can_format_item(item) {
            continue;
        }

        // Get the module path (all elements except the last one)
        let module_path = if let Some(path) = item_paths.get(id) {
            if path.len() > 1 {
                path[..path.len()-1].join("::")
            } else {
                continue; // Skip root-level items without module
            }
        } else {
            continue; // Skip items without path info
        };

        modules.entry(module_path)
            .or_default()
            .push((*id, item.clone()));
    }

    // Sort items within each module by name
    for items in modules.values_mut() {
        items.sort_by(|a, b| {
            let name_a = a.1.name.as_deref().unwrap_or("");
            let name_b = b.1.name.as_deref().unwrap_or("");
            name_a.cmp(name_b)
        });
    }

    modules
}

fn can_format_item(item: &Item) -> bool {
    matches!(
        item.inner,
        ItemEnum::Struct(_) | ItemEnum::Enum(_) | ItemEnum::Function(_) |
        ItemEnum::Trait(_) | ItemEnum::Module(_) | ItemEnum::Constant { .. } |
        ItemEnum::TypeAlias(_)
    )
}

fn generate_toc(modules: &HashMap<String, Vec<(Id, Item)>>, crate_name: &str) -> String {
    let mut toc = String::new();

    // Sort modules alphabetically
    let mut module_names: Vec<_> = modules.keys().collect();
    module_names.sort();

    for module_name in module_names {
        let items = &modules[module_name];

        // Get the last component of the module path for display
        let display_name = module_name.strip_prefix(&format!("{}::", crate_name))
            .unwrap_or(module_name);

        toc.push_str(&format!("- **{}**\n", display_name));

        for (_id, item) in items {
            if let Some(name) = &item.name {
                let full_path = format!("{}::{}", module_name, name);
                let anchor = full_path.to_lowercase().replace("::", "-");
                toc.push_str(&format!("  - [{}](#{})\n", name, anchor));
            }
        }
    }

    toc
}

fn generate_content(
    modules: &HashMap<String, Vec<(Id, Item)>>,
    crate_data: &Crate,
    item_paths: &HashMap<Id, Vec<String>>,
) -> String {
    let mut output = String::new();

    // Sort modules alphabetically
    let mut module_names: Vec<_> = modules.keys().collect();
    module_names.sort();

    for module_name in module_names {
        let items = &modules[module_name];

        // Module header
        output.push_str(&format!("# Module: `{}`\n\n", module_name));

        // Generate content for each item in the module
        for (id, item) in items {
            if let Some(section) = format_item_with_path(id, item, crate_data, item_paths) {
                output.push_str(&section);
                output.push_str("\n\n");
            }
        }

        output.push_str("---\n\n");
    }

    output
}

fn format_item_with_path(
    item_id: &Id,
    item: &Item,
    crate_data: &Crate,
    item_paths: &HashMap<Id, Vec<String>>,
) -> Option<String> {
    let full_path = item_paths.get(item_id)?;
    let full_name = full_path.join("::");

    let mut output = format_item(item_id, item, crate_data)?;

    // Replace the simple name header with the full path
    if let Some(name) = &item.name {
        let old_header = format!("## {}\n\n", name);
        let new_header = format!("## {}\n\n", full_name);
        output = output.replace(&old_header, &new_header);
    }

    Some(output)
}

fn is_public(item: &Item) -> bool {
    matches!(item.visibility, Visibility::Public)
}

fn format_item(item_id: &rustdoc_types::Id, item: &Item, crate_data: &Crate) -> Option<String> {
    let name = item.name.as_ref()?;
    let mut output = String::new();

    match &item.inner {
        ItemEnum::Struct(s) => {
            output.push_str(&format!("## {}\n\n", name));
            output.push_str("**Type:** Struct\n\n");

            if let Some(docs) = &item.docs {
                output.push_str(&format!("{}\n\n", docs));
            }

            if !s.generics.params.is_empty() {
                output.push_str("**Generic Parameters:**\n");
                for param in &s.generics.params {
                    output.push_str(&format!("- {}\n", format_generic_param(param)));
                }
                output.push('\n');
            }

            match &s.kind {
                rustdoc_types::StructKind::Plain { fields, .. } => {
                    if !fields.is_empty() {
                        output.push_str("**Fields:**\n\n");
                        output.push_str("| Name | Type | Description |\n");
                        output.push_str("|------|------|-------------|\n");
                        for field_id in fields {
                            if let Some(field) = crate_data.index.get(field_id) {
                                if let Some(field_name) = &field.name {
                                    let field_type = if let ItemEnum::StructField(ty) = &field.inner {
                                        format_type(ty)
                                    } else {
                                        "?".to_string()
                                    };
                                    let field_doc = if let Some(docs) = &field.docs {
                                        docs.lines().next().unwrap_or("").to_string()
                                    } else {
                                        "".to_string()
                                    };
                                    output.push_str(&format!("| `{}` | `{}` | {} |\n",
                                        field_name, field_type, field_doc));
                                }
                            }
                        }
                        output.push('\n');
                    }
                }
                rustdoc_types::StructKind::Tuple(fields) => {
                    output.push_str(&format!("**Tuple Struct** with {} field(s)\n\n", fields.len()));
                }
                rustdoc_types::StructKind::Unit => {
                    output.push_str("**Unit Struct**\n\n");
                }
            }

            let (inherent_impls, trait_impls) = collect_impls_for_type(item_id, crate_data);

            if !inherent_impls.is_empty() {
                output.push_str("**Methods:**\n\n");
                for impl_block in inherent_impls {
                    output.push_str(&format_impl_methods(impl_block, crate_data));
                }
                output.push('\n');
            }

            if !trait_impls.is_empty() {
                let user_impls: Vec<_> = trait_impls.iter()
                    .filter(|impl_block| {
                        !impl_block.is_synthetic && impl_block.blanket_impl.is_none()
                    })
                    .collect();

                if !user_impls.is_empty() {
                    output.push_str("**Trait Implementations:**\n\n");
                    for impl_block in user_impls {
                        if let Some(trait_ref) = &impl_block.trait_ {
                            output.push_str(&format!("- **{}**\n", trait_ref.path));
                            let methods = format_impl_methods(impl_block, crate_data);
                            if !methods.is_empty() {
                                for line in methods.lines() {
                                    output.push_str(&format!("  {}\n", line));
                                }
                            }
                        }
                    }
                    output.push('\n');
                }
            }
        }
        ItemEnum::Enum(e) => {
            output.push_str(&format!("## {}\n\n", name));
            output.push_str("**Type:** Enum\n\n");

            if let Some(docs) = &item.docs {
                output.push_str(&format!("{}\n\n", docs));
            }

            if !e.generics.params.is_empty() {
                output.push_str("**Generic Parameters:**\n");
                for param in &e.generics.params {
                    output.push_str(&format!("- {}\n", format_generic_param(param)));
                }
                output.push('\n');
            }

            if !e.variants.is_empty() {
                output.push_str("**Variants:**\n\n");
                output.push_str("| Variant | Kind | Description |\n");
                output.push_str("|---------|------|-------------|\n");
                for variant_id in &e.variants {
                    if let Some(variant) = crate_data.index.get(variant_id) {
                        if let Some(variant_name) = &variant.name {
                            let variant_kind = if let ItemEnum::Variant(v) = &variant.inner {
                                match &v.kind {
                                    rustdoc_types::VariantKind::Plain => "Unit".to_string(),
                                    rustdoc_types::VariantKind::Tuple(fields) => {
                                        let types: Vec<_> = fields.iter().map(|field_id| {
                                            if let Some(id) = field_id {
                                                if let Some(field_item) = crate_data.index.get(id) {
                                                    if let ItemEnum::StructField(ty) = &field_item.inner {
                                                        return format_type(ty);
                                                    }
                                                }
                                            }
                                            "?".to_string()
                                        }).collect();
                                        format!("Tuple({})", types.join(", "))
                                    },
                                    rustdoc_types::VariantKind::Struct { fields, .. } => {
                                        format!("Struct ({} fields)", fields.len())
                                    }
                                }
                            } else {
                                "?".to_string()
                            };
                            let variant_doc = if let Some(docs) = &variant.docs {
                                docs.lines().next().unwrap_or("").to_string()
                            } else {
                                "".to_string()
                            };
                            output.push_str(&format!("| `{}` | {} | {} |\n",
                                variant_name, variant_kind, variant_doc));
                        }
                    }
                }
                output.push('\n');
            }

            let (inherent_impls, trait_impls) = collect_impls_for_type(item_id, crate_data);

            if !inherent_impls.is_empty() {
                output.push_str("**Methods:**\n\n");
                for impl_block in inherent_impls {
                    output.push_str(&format_impl_methods(impl_block, crate_data));
                }
                output.push('\n');
            }

            if !trait_impls.is_empty() {
                let user_impls: Vec<_> = trait_impls.iter()
                    .filter(|impl_block| {
                        !impl_block.is_synthetic && impl_block.blanket_impl.is_none()
                    })
                    .collect();

                if !user_impls.is_empty() {
                    output.push_str("**Trait Implementations:**\n\n");
                    for impl_block in user_impls {
                        if let Some(trait_ref) = &impl_block.trait_ {
                            output.push_str(&format!("- **{}**\n", trait_ref.path));
                            let methods = format_impl_methods(impl_block, crate_data);
                            if !methods.is_empty() {
                                for line in methods.lines() {
                                    output.push_str(&format!("  {}\n", line));
                                }
                            }
                        }
                    }
                    output.push('\n');
                }
            }
        }
        ItemEnum::Function(f) => {
            output.push_str(&format!("## {}\n\n", name));
            output.push_str("**Type:** Function\n\n");

            if let Some(docs) = &item.docs {
                output.push_str(&format!("{}\n\n", docs));
            }

            output.push_str("```rust\n");
            output.push_str(&format!("fn {}", name));

            if !f.generics.params.is_empty() {
                output.push('<');
                let params: Vec<String> = f.generics.params.iter()
                    .map(format_generic_param)
                    .collect();
                output.push_str(&params.join(", "));
                output.push('>');
            }

            output.push('(');
            let inputs: Vec<String> = f.sig.inputs.iter()
                .map(|(name, ty)| format!("{}: {}", name, format_type(ty)))
                .collect();
            output.push_str(&inputs.join(", "));
            output.push(')');

            if let Some(output_type) = &f.sig.output {
                output.push_str(&format!(" -> {}", format_type(output_type)));
            }

            output.push_str("\n```\n\n");
        }
        ItemEnum::Trait(t) => {
            output.push_str(&format!("## {}\n\n", name));
            output.push_str("**Type:** Trait\n\n");

            if let Some(docs) = &item.docs {
                output.push_str(&format!("{}\n\n", docs));
            }

            if !t.items.is_empty() {
                output.push_str("**Methods:**\n\n");
                for method_id in &t.items {
                    if let Some(method) = crate_data.index.get(method_id) {
                        if let Some(method_name) = &method.name {
                            output.push_str(&format!("- `{}`", method_name));
                            if let Some(method_docs) = &method.docs {
                                output.push_str(&format!(": {}", method_docs.lines().next().unwrap_or("")));
                            }
                            output.push('\n');
                        }
                    }
                }
                output.push('\n');
            }
        }
        ItemEnum::Module(_) => {
            output.push_str(&format!("## Module: {}\n\n", name));

            if let Some(docs) = &item.docs {
                output.push_str(&format!("{}\n\n", docs));
            }
        }
        ItemEnum::Constant { .. } => {
            output.push_str(&format!("## {}\n\n", name));
            output.push_str("**Type:** Constant\n\n");

            if let Some(docs) = &item.docs {
                output.push_str(&format!("{}\n\n", docs));
            }
        }
        ItemEnum::TypeAlias(_) => {
            output.push_str(&format!("## {}\n\n", name));
            output.push_str("**Type:** Type Alias\n\n");

            if let Some(docs) = &item.docs {
                output.push_str(&format!("{}\n\n", docs));
            }
        }
        _ => {
            return None;
        }
    }

    Some(output)
}

fn format_generic_param(param: &rustdoc_types::GenericParamDef) -> String {
    match &param.kind {
        rustdoc_types::GenericParamDefKind::Lifetime { .. } => {
            format!("'{}", param.name)
        }
        rustdoc_types::GenericParamDefKind::Type { .. } => {
            param.name.clone()
        }
        rustdoc_types::GenericParamDefKind::Const { .. } => {
            format!("const {}", param.name)
        }
    }
}

fn collect_impls_for_type<'a>(type_id: &rustdoc_types::Id, crate_data: &'a Crate) -> (Vec<&'a rustdoc_types::Impl>, Vec<&'a rustdoc_types::Impl>) {
    use rustdoc_types::Type;

    let mut inherent_impls = Vec::new();
    let mut trait_impls = Vec::new();

    for item in crate_data.index.values() {
        if let ItemEnum::Impl(impl_block) = &item.inner {
            let matches = match &impl_block.for_ {
                Type::ResolvedPath(path) => path.id == *type_id,
                _ => false,
            };

            if matches {
                if impl_block.trait_.is_some() {
                    trait_impls.push(impl_block);
                } else {
                    inherent_impls.push(impl_block);
                }
            }
        }
    }

    (inherent_impls, trait_impls)
}

fn format_impl_methods(impl_block: &rustdoc_types::Impl, crate_data: &Crate) -> String {
    let mut output = String::new();

    for method_id in &impl_block.items {
        if let Some(method) = crate_data.index.get(method_id) {
            if let ItemEnum::Function(f) = &method.inner {
                if let Some(method_name) = &method.name {
                    let sig = format_function_signature(method_name, f);
                    let doc = if let Some(docs) = &method.docs {
                        docs.lines().next().unwrap_or("")
                    } else {
                        ""
                    };
                    output.push_str(&format!("- `{}` - {}\n", sig, doc));
                }
            }
        }
    }

    output
}

fn format_function_signature(name: &str, f: &rustdoc_types::Function) -> String {
    let mut sig = format!("fn {}", name);

    if !f.generics.params.is_empty() {
        sig.push('<');
        let params: Vec<String> = f.generics.params.iter()
            .map(format_generic_param)
            .collect();
        sig.push_str(&params.join(", "));
        sig.push('>');
    }

    sig.push('(');
    let inputs: Vec<String> = f.sig.inputs.iter()
        .map(|(name, ty)| format!("{}: {}", name, format_type(ty)))
        .collect();
    sig.push_str(&inputs.join(", "));
    sig.push(')');

    if let Some(output_type) = &f.sig.output {
        sig.push_str(&format!(" -> {}", format_type(output_type)));
    }

    sig
}

fn format_type(ty: &rustdoc_types::Type) -> String {
    use rustdoc_types::Type;
    match ty {
        Type::ResolvedPath(path) => path.path.clone(),
        Type::DynTrait(dt) => {
            if let Some(first) = dt.traits.first() {
                format!("dyn {}", first.trait_.path)
            } else {
                "dyn Trait".to_string()
            }
        }
        Type::Generic(name) => name.clone(),
        Type::Primitive(name) => name.clone(),
        Type::FunctionPointer(_) => "fn(...)".to_string(),
        Type::Tuple(types) => {
            let formatted: Vec<_> = types.iter().map(format_type).collect();
            format!("({})", formatted.join(", "))
        }
        Type::Slice(inner) => format!("[{}]", format_type(inner)),
        Type::Array { type_, len } => format!("[{}; {}]", format_type(type_), len),
        Type::Pat { type_, .. } => format_type(type_),
        Type::ImplTrait(_bounds) => "impl Trait".to_string(),
        Type::Infer => "_".to_string(),
        Type::RawPointer { is_mutable, type_ } => {
            if *is_mutable {
                format!("*mut {}", format_type(type_))
            } else {
                format!("*const {}", format_type(type_))
            }
        }
        Type::BorrowedRef { lifetime, is_mutable, type_ } => {
            let lifetime_str = lifetime.as_deref().unwrap_or("'_");
            if *is_mutable {
                format!("&{} mut {}", lifetime_str, format_type(type_))
            } else {
                format!("&{} {}", lifetime_str, format_type(type_))
            }
        }
        Type::QualifiedPath { name, self_type, trait_, .. } => {
            if let Some(trait_) = trait_ {
                format!("<{} as {}>::{}", format_type(self_type), trait_.path, name)
            } else {
                format!("{}::{}", format_type(self_type), name)
            }
        }
    }
}

fn generate_crate_index(
    crate_name: &str,
    root_item: &Item,
    modules: &HashMap<String, Vec<(Id, Item)>>,
) -> String {
    let mut output = String::new();

    output.push_str(&format!("# {}\n\n", crate_name));

    if let Some(docs) = &root_item.docs {
        output.push_str(&format!("{}\n\n", docs));
    }

    // Module listing with summary
    output.push_str("## Modules\n\n");

    let mut module_names: Vec<_> = modules.keys().collect();
    module_names.sort();

    for module_name in module_names {
        let items = &modules[module_name];

        let display_name = module_name.strip_prefix(&format!("{}::", crate_name))
            .unwrap_or(module_name);

        let module_file = format!("{}.md", display_name.replace("::", "/"));

        // Count item types
        let mut counts = HashMap::new();
        for (_id, item) in items {
            let type_name = match &item.inner {
                ItemEnum::Struct(_) => "structs",
                ItemEnum::Enum(_) => "enums",
                ItemEnum::Function(_) => "functions",
                ItemEnum::Trait(_) => "traits",
                ItemEnum::Constant { .. } => "constants",
                ItemEnum::TypeAlias(_) => "type aliases",
                ItemEnum::Module(_) => "modules",
                _ => continue,
            };
            *counts.entry(type_name).or_insert(0) += 1;
        }

        output.push_str(&format!("### [`{}`]({})\n\n", display_name, module_file));

        if !counts.is_empty() {
            let summary: Vec<String> = counts.iter()
                .map(|(name, count)| format!("{} {}", count, name))
                .collect();
            output.push_str(&format!("*{}*\n\n", summary.join(", ")));
        }
    }

    output
}

fn generate_module_file(
    module_name: &str,
    items: &[(Id, Item)],
    crate_data: &Crate,
    item_paths: &HashMap<Id, Vec<String>>,
    crate_name: &str,
) -> String {
    let mut output = String::new();

    let display_name = module_name.strip_prefix(&format!("{}::", crate_name))
        .unwrap_or(module_name);

    // Breadcrumb
    let breadcrumb = module_name.replace("::", " > ");
    output.push_str(&format!("**{}**\n\n", breadcrumb));

    output.push_str(&format!("# Module: {}\n\n", display_name));

    // Table of contents for this module
    output.push_str("## Contents\n\n");

    let mut by_type: HashMap<&str, Vec<&Item>> = HashMap::new();
    for (_id, item) in items {
        let type_name = match &item.inner {
            ItemEnum::Struct(_) => "Structs",
            ItemEnum::Enum(_) => "Enums",
            ItemEnum::Function(_) => "Functions",
            ItemEnum::Trait(_) => "Traits",
            ItemEnum::Constant { .. } => "Constants",
            ItemEnum::TypeAlias(_) => "Type Aliases",
            ItemEnum::Module(_) => "Modules",
            _ => continue,
        };
        by_type.entry(type_name).or_default().push(item);
    }

    let type_order = ["Modules", "Structs", "Enums", "Functions", "Traits", "Constants", "Type Aliases"];
    for type_name in &type_order {
        if let Some(items_of_type) = by_type.get(type_name) {
            output.push_str(&format!("**{}**\n\n", type_name));
            for item in items_of_type {
                if let Some(name) = &item.name {
                    let anchor = name.to_lowercase();
                    output.push_str(&format!("- [`{}`](#{})", name, anchor));
                    if let Some(docs) = &item.docs {
                        if let Some(first_line) = docs.lines().next() {
                            if !first_line.is_empty() {
                                output.push_str(&format!(" - {}", first_line));
                            }
                        }
                    }
                    output.push('\n');
                }
            }
            output.push('\n');
        }
    }

    output.push_str("---\n\n");

    // Generate content for each item
    for (id, item) in items {
        if let Some(section) = format_item_with_path(id, item, crate_data, item_paths) {
            output.push_str(&section);
            output.push_str("\n\n");
        }
    }

    output
}
