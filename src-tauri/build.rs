use anyhow::{bail, Result};
use std::path::Path;
use std::{collections::HashSet, fs};
use syn::{File, FnArg, Item, ItemFn, Pat, PatType, Type, TypePath};
use walkdir::WalkDir;

const RS_COMMAND_PATH: &str = "./src/commands/";
const TS_CODE_PATH: &str = "../src/tauri/";

fn main() -> Result<()> {
    // Generate TypeScript code
    let command_file_paths = find_command_files(RS_COMMAND_PATH);
    for command_file_path in command_file_paths {
        let command_file_name = file_name_without_extension(&command_file_path)?;
        generate_typescript_api(&command_file_path, &command_file_name)?;
    }

    // Build Tauri
    tauri_build::build();

    // Force admin on Windows
    #[cfg(all(windows, not(feature = "no-admin")))]
    {
        embed_resource::compile("assets/windows/admin.rc");
    }

    Ok(())
}

fn file_name_without_extension(file_path: &str) -> Result<String> {
    let path = Path::new(file_path);

    if let Some(stem) = path.file_stem() {
        if let Some(stem_str) = stem.to_str() {
            Ok(stem_str.to_string())
        } else {
            bail!("Filename stem is not valid UTF-8")
        }
    } else {
        bail!("No filename stem found in the path")
    }
}

fn find_command_files(source_dir: &str) -> Vec<String> {
    let mut command_files = Vec::new();

    // Walk through the directory to find all .rs files
    for entry in WalkDir::new(source_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
            let file_path = entry.path().display().to_string();

            // Read and parse the Rust file
            let content = fs::read_to_string(&file_path).expect("Failed to read file");
            let syntax = syn::parse_file(&content).expect("Failed to parse Rust file");

            // Check for #[command] attribute
            if contains_command_attribute(&syntax) {
                command_files.push(file_path);
            }
        }
    }

    command_files
}

fn contains_command_attribute(syntax: &File) -> bool {
    // Iterate through each item in the Rust file
    for item in &syntax.items {
        if let Item::Fn(ItemFn { attrs, .. }) = item {
            // Check if any attribute is #[command]
            if attrs.iter().any(|attr| attr.path().is_ident("command")) {
                return true;
            }
        }
    }
    false
}

fn generate_typescript_api(command_file_path: &str, ts_file_name: &str) -> Result<()> {
    let content = fs::read_to_string(command_file_path).expect("Failed to read source file");
    let syntax: File = syn::parse_file(&content).expect("Failed to parse file");

    let mut ts_types = String::new();
    let mut ts_functions = String::new();
    let mut seen_imports: HashSet<String> = HashSet::new(); // Track seen types for imports

    ts_types.push_str("import { invoke } from '@tauri-apps/api/core';\n");

    for item in syntax.items {
        if let syn::Item::Fn(ItemFn { attrs, sig, .. }) = item {
            if attrs.iter().any(|attr| attr.path().is_ident("command")) {
                let fn_name = sig.ident.to_string();

                // Collect parameters, ignoring those named "storage"
                let params: Vec<(String, String)> = sig
                    .inputs
                    .iter()
                    .filter_map(|arg| {
                        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
                            if let Pat::Ident(ident) = &**pat {
                                if ident.ident != "storage" {
                                    let type_str = type_to_ts(ty, &mut seen_imports, &mut ts_types);
                                    return Some((ident.ident.to_string(), type_str));
                                }
                            }
                        }
                        None
                    })
                    .collect();

                // Create TypeScript parameter list and types
                let ts_param_list: String = params
                    .iter()
                    .map(|(param_name, param_type)| format!("{}: {}", param_name, param_type))
                    .collect::<Vec<String>>()
                    .join(", ");

                ts_functions.push_str(&format!(
                    "export async function {}({}) {{\n",
                    fn_name, ts_param_list
                ));

                // Generate invoke parameter object
                let invoke_param_obj = if params.is_empty() {
                    "{}".to_string()
                } else {
                    let param_names = params
                        .iter()
                        .map(|(param_name, _)| param_name.clone())
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{{ {} }}", param_names)
                };

                ts_functions.push_str(&format!(
                    "  return await invoke('{}', {});\n",
                    fn_name, invoke_param_obj
                ));
                ts_functions.push_str("}\n\n");
            }
        }
    }

    let ts_file_path = format!("{TS_CODE_PATH}{ts_file_name}.ts");
    eprintln!("Writing: {}", ts_file_path);

    let combined = format!("{}\n\n{}", ts_types.trim_end(), ts_functions.trim_end());
    fs::write(ts_file_path, combined).expect("Failed to write TypeScript bindings");

    println!("cargo:rerun-if-changed={}", command_file_path);

    Ok(())
}

// Helper function for converting Rust types to TypeScript types
fn type_to_ts(ty: &Type, seen_imports: &mut HashSet<String>, ts_types: &mut String) -> String {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            let rust_type = path.segments.last().unwrap().ident.to_string();
            match rust_type.as_str() {
                "String" => "string".to_string(),
                "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "usize" | "isize"
                | "f32" | "f64" => "number".to_string(),
                "bool" => "boolean".to_string(),
                "Vec" => {
                    if let syn::PathArguments::AngleBracketed(args) =
                        &path.segments.last().unwrap().arguments
                    {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                            return format!(
                                "Array<{}>",
                                type_to_ts(inner_ty, seen_imports, ts_types)
                            );
                        }
                    }
                    "Array<any>".to_string()
                }
                "Option" => {
                    if let syn::PathArguments::AngleBracketed(args) =
                        &path.segments.last().unwrap().arguments
                    {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                            let inner_ts_type = type_to_ts(inner_ty, seen_imports, ts_types);
                            return format!("{} | undefined", inner_ts_type);
                        }
                    }
                    "any".to_string()
                }
                _ => {
                    if !seen_imports.contains(&rust_type) {
                        seen_imports.insert(rust_type.clone());
                        ts_types
                            .push_str(&format!("import {{ {} }} from \"./types\";\n", rust_type));
                    }
                    rust_type
                }
            }
        }
        _ => "any".to_string(),
    }
}
