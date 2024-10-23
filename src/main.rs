/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/cargo-set-version
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use anyhow::{anyhow, Result};
use cargo_metadata::MetadataCommand;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use toml_edit::{DocumentMut, Value};

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();

    // this is needed for it to act as a cargo-plugin
    if args.len() > 1 && args[1] == "set-version" {
        args.remove(1);
    }

    if args.len() != 2 {
        eprintln!("Usage: {} <new_version>", args[0]);
        std::process::exit(1);
    }

    let new_version = &args[1];

    let metadata = MetadataCommand::new().no_deps().exec()?;

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
        let content = fs::read_to_string(&manifest_path)?;
        let mut doc = content.parse::<DocumentMut>()?;

        let mut updated = false;

        if let Some(package_table) = doc.get_mut("package") {
            if let Some(version) = package_table.get_mut("version") {
                *version = toml_edit::value(new_version.to_string());
                updated = true;
            }
        } else {
            return Err(anyhow!("could not find package section in Cargo.toml!"));
        }

        let dependency_sections = ["dependencies", "dev-dependencies", "build-dependencies"];

        for section in &dependency_sections {
            if let Some(dependencies) = doc.get_mut(*section) {
                if let Some(table) = dependencies.as_table_like_mut() {
                    for (dep_name, dep_item) in table.iter_mut() {
                        if package_names.contains(dep_name.get()) {
                            if let Some(dep_table) = dep_item.as_inline_table_mut() {
                                dep_table.insert("version", Value::from(new_version.to_string()));
                                updated = true;
                            } else if dep_item.is_str() {
                                *dep_item = toml_edit::value(new_version.to_string());
                                updated = true;
                            }
                        }
                    }
                }
            }
        }

        if updated {
            fs::write(&manifest_path, doc.to_string())?;
            println!("Updated version in: {}", manifest_path);
        }
    }

    Ok(())
}
