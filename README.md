# 🎉 Cargo Set Version

[![Crates.io](https://img.shields.io/crates/v/cargo-set-version)](https://crates.io/cargo-set-version)

> **Automatically update versions for all crates in your Rust workspace with ease!** 🦀✨

## ✨ Features

- **Bulk Version Update**: Change the version of all your workspace crates in one command!
- **Seamless Integration**: Works as a Cargo plugin, making it a natural part of your Rust workflow.
- **Dependency Management**: Automatically updates inter-crate dependencies to match the new version.

## 📦 Installation

To install `cargo-set-version`, ensure you have [Rust and Cargo](https://rustup.rs/) installed, then run:

```bash
cargo install cargo-set-version
```

## 🛠️ Usage

Navigate to the root of your Rust workspace and run:

```bash
cargo set-version <new_version>
```

### Example

Suppose you want to update all your workspace crates to version 1.2.3. Simply execute:

```bash
cargo set-version 1.2.3
```
