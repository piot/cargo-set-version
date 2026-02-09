/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/cargo-set-version
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use anyhow::{Result, anyhow};
use cargo_metadata::MetadataCommand;
use cargo_set_version::{ensure_version_increase, parse_new_version};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::PathBuf;
use toml_edit::{DocumentMut, Value};

struct Arguments {
    pub manifest_path: Option<PathBuf>,
    pub new_version: String,
}

// TODO: Maybe use clap or similar instead of manual parsing?
fn parse_arguments(in_args: &[String]) -> Option<Arguments> {
    let mut args = in_args.to_vec();
    let mut manifest_path = None;
    let mut i = 1;

    if in_args.len() < 2 {
        return None;
    }

    while i < args.len() {
        if args[i] == "--manifest-path" {
            if i + 1 < args.len() {
                manifest_path = Some(PathBuf::from(&args[i + 1]));
                args.remove(i);
                args.remove(i);
            } else {
                eprintln!("Error: --manifest-path requires a value");
                return None;
            }
        } else {
            i += 1;
        }
    }

    if args.len() < 2 {
        return None;
    }

    let new_version = args[1].clone();

    Some(Arguments {
        manifest_path,
        new_version,
    })
}

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();

    // this is needed for it to act as a cargo-plugin
    if args.len() > 1 && args[1] == "set-version" {
        args.remove(1);
    }

    let Some(arguments) = parse_arguments(&args) else {
        eprintln!(
            "Usage: {} [--manifest-path some/crate/Cargo.toml] <new_version>",
            args[0]
        );
        std::process::exit(1);
    };

    let new_version = parse_new_version(&arguments.new_version)?;

    let mut start = MetadataCommand::new();
    let cmd = if let Some(path) = arguments.manifest_path {
        start.manifest_path(path)
    } else {
        start.no_deps()
    };
    let metadata = cmd.exec()?;

    let mut package_names = HashSet::new();
    let mut package_manifest_paths = HashMap::new();

    for package in &metadata.packages {
        if !metadata.workspace_members.contains(&package.id) {
            continue;
        }

        package_names.insert(package.name.clone());
        package_manifest_paths.insert(package.name.clone(), package.manifest_path.clone());
    }

    for package in &metadata.packages {
        if !metadata.workspace_members.contains(&package.id) {
            continue;
        }

        let manifest_path = &package.manifest_path;
        let content = fs::read_to_string(manifest_path)?;
        let mut doc = content.parse::<DocumentMut>()?;

        let mut updated = false;

        if let Some(package_table) = doc.get_mut("package") {
            if let Some(version) = package_table.get_mut("version") {
                ensure_version_increase(&new_version, &package.version, &package.name)?;
                *version = toml_edit::value(new_version.to_string());
                updated = true;
            }
        } else {
            return Err(anyhow!("could not find package section in Cargo.toml!"));
        }

        let dependency_sections = ["dependencies", "dev-dependencies", "build-dependencies"];

        for section in &dependency_sections {
            if let Some(dependencies) = doc.get_mut(section) {
                if let Some(table) = dependencies.as_table_like_mut() {
                    for (dep_name, dep_item) in table.iter_mut() {
                        if package_names.contains(dep_name.get()) {
                            if let Some(dep_table) = dep_item.as_inline_table_mut() {
                                dep_table.insert(
                                    "version",
                                    Value::from(arguments.new_version.to_string()),
                                );
                                updated = true;
                            } else if dep_item.is_str() {
                                *dep_item = toml_edit::value(arguments.new_version.to_string());
                                updated = true;
                            }
                        }
                    }
                }
            }
        }

        if updated {
            fs::write(manifest_path, doc.to_string())?;
            println!("Updated version in: {}", manifest_path);
        }
    }

    Ok(())
}
