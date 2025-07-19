use anyhow::Result;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use syn::{
    File, FnArg, GenericArgument, Item, ItemFn, Pat, PatType, PathArguments, ReturnType, Type,
    TypePath,
};
use walkdir::WalkDir;

const RS_COMMAND_PATH: &str = "./src/commands/";
const TS_CODE_PATH: &str = "../src/tauri/";

fn main() -> Result<()> {
    let command_file_paths = find_command_files(RS_COMMAND_PATH);
    for command_file_path in command_file_paths {
        let ts_file_name = file_name_without_extension(&command_file_path)?;
        generate_typescript_api(&command_file_path, &ts_file_name)?;
    }
    tauri_build::build();

    #[cfg(all(windows, not(feature = "no-admin")))]
    {
        embed_resource::compile("assets/windows/admin.rc");
    }

    Ok(())
}

fn file_name_without_extension(file_path: &str) -> Result<String> {
    Path::new(file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("Invalid file name: {}", file_path))
}

fn find_command_files(source_dir: &str) -> Vec<String> {
    WalkDir::new(source_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("rs"))
        .filter_map(|entry| {
            let file_path = entry.path().display().to_string();
            let content = fs::read_to_string(&file_path).ok()?;
            let syntax = syn::parse_file(&content).ok()?;
            if contains_command_attribute(&syntax) {
                Some(file_path)
            } else {
                None
            }
        })
        .collect()
}

fn contains_command_attribute(syntax: &File) -> bool {
    syntax.items.iter().any(|item| {
        if let Item::Fn(ItemFn { attrs, .. }) = item {
            attrs.iter().any(|attr| attr.path().is_ident("command"))
        } else {
            false
        }
    })
}

fn generate_typescript_api(command_file_path: &str, ts_file_name: &str) -> Result<()> {
    let content = fs::read_to_string(command_file_path)?;
    let syntax: File = syn::parse_file(&content)?;

    let mut ts_types = String::from("import { invoke } from '@tauri-apps/api/core';\n");
    let mut seen_imports: HashSet<String> = HashSet::new();
    let mut ts_functions = String::new();

    for item in syntax.items {
        if let Item::Fn(ItemFn { attrs, sig, .. }) = item {
            if !attrs.iter().any(|attr| attr.path().is_ident("command")) {
                continue;
            }
            let fn_name = sig.ident.to_string();

            let params: Vec<(String, String)> = sig
                .inputs
                .iter()
                .filter_map(|arg| {
                    if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
                        if let Pat::Ident(ident) = &**pat {
                            if ident.ident != "storage" {
                                let ty_ts = type_to_ts(ty, &mut seen_imports, &mut ts_types)
                                    .unwrap_or_else(|| "any".to_string());
                                return Some((ident.ident.to_string(), ty_ts));
                            }
                        }
                    }
                    None
                })
                .collect();

            let ts_param_list: String = params
                .iter()
                .map(|(name, ty)| format!("{}: {}", name, ty))
                .collect::<Vec<_>>()
                .join(", ");

            // Determine if return is void (Promise<void>)
            let is_promise_void = returns_unit_or_result_unit(&sig.output);

            // Determine return type for TypeScript (without Promise<void>)
            let ts_return_type = if is_promise_void {
                String::new()
            } else if let ReturnType::Type(_, ty) = &sig.output {
                let ty = extract_result_ok_type(ty).unwrap_or(&**ty);
                let ts_type = type_to_ts(ty, &mut seen_imports, &mut ts_types)
                    .unwrap_or_else(|| "any".to_string());
                format!(": Promise<{}>", ts_type)
            } else {
                String::new()
            };

            ts_functions.push_str(&format!(
                "export async function {}({}){} {{\n",
                fn_name, ts_param_list, ts_return_type
            ));

            let invoke_param_obj = if params.is_empty() {
                "{}".to_string()
            } else {
                let param_names = params
                    .iter()
                    .map(|(name, _)| name)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{ {} }}", param_names)
            };

            // Only use 'return await' for non-voids
            if is_promise_void {
                ts_functions.push_str(&format!(
                    "  await invoke('{}', {});\n",
                    fn_name, invoke_param_obj
                ));
            } else {
                ts_functions.push_str(&format!(
                    "  return await invoke('{}', {});\n",
                    fn_name, invoke_param_obj
                ));
            }

            ts_functions.push_str("}\n\n");
        }
    }

    let ts_file_path = format!("{TS_CODE_PATH}{ts_file_name}.ts");
    eprintln!("Writing: {}", ts_file_path);

    let combined = format!("{}\n\n{}", ts_types.trim_end(), ts_functions.trim_end());
    fs::write(ts_file_path, combined)?;
    println!("cargo:rerun-if-changed={}", command_file_path);

    Ok(())
}

fn returns_unit_or_result_unit(output: &ReturnType) -> bool {
    match output {
        ReturnType::Default => true,
        ReturnType::Type(_, ty) => {
            is_unit_type(ty)
                || extract_result_ok_type(ty).map_or(false, |ok_ty| is_unit_type(ok_ty))
        }
    }
}

fn extract_result_ok_type(ty: &Type) -> Option<&Type> {
    let type_path = match ty {
        Type::Path(type_path) => type_path,
        _ => return None,
    };

    let segment = type_path.path.segments.last()?;
    if segment.ident != "Result" {
        return None;
    }

    let args = match &segment.arguments {
        PathArguments::AngleBracketed(args) => args,
        _ => return None,
    };

    args.args.iter().find_map(|arg| {
        if let GenericArgument::Type(ok_ty) = arg {
            Some(ok_ty)
        } else {
            None
        }
    })
}

fn is_unit_type(ty: &Type) -> bool {
    matches!(ty, Type::Tuple(tuple) if tuple.elems.is_empty())
}

// Rust to TypeScript type conversion
fn type_to_ts(
    ty: &Type,
    seen_imports: &mut HashSet<String>,
    ts_types: &mut String,
) -> Option<String> {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            let rust_type = path.segments.last().unwrap().ident.to_string();
            match rust_type.as_str() {
                "String" => Some("string".to_string()),
                "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "usize" | "isize"
                | "f32" | "f64" => Some("number".to_string()),
                "bool" => Some("boolean".to_string()),
                "Vec" => {
                    if let PathArguments::AngleBracketed(args) =
                        &path.segments.last().unwrap().arguments
                    {
                        if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                            return type_to_ts(inner_ty, seen_imports, ts_types)
                                .map(|inner| format!("Array<{}>", inner));
                        }
                    }
                    None
                }
                "Option" => {
                    if let PathArguments::AngleBracketed(args) =
                        &path.segments.last().unwrap().arguments
                    {
                        if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                            return type_to_ts(inner_ty, seen_imports, ts_types)
                                .map(|inner_ts_type| format!("{} | undefined", inner_ts_type));
                        }
                    }
                    None
                }
                _ => {
                    if !seen_imports.contains(&rust_type) {
                        seen_imports.insert(rust_type.clone());
                        ts_types
                            .push_str(&format!("import {{ {} }} from \"./types\";\n", rust_type));
                    }
                    Some(rust_type)
                }
            }
        }
        _ => None,
    }
}
