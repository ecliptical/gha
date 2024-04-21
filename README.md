# Utilities for implementing GitHub Actions

This crate provides basic utilities that help you develop [custom GitHub Actions](https://docs.github.com/en/actions/creating-actions/about-custom-actions) in Rust.

## Environment Variables

```rust
use gha::github_workspace;
use std::fs::File;

let workspace = github_workspace();
let my_file = File::open(workspace.append("my_file.yaml"))?
```

## Messages

Easily generate [workflow commands](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions); e.g.,

```rust
use gha::{debug, error};

debug!("current dir: {:#?}", std::env::current_dir());
error!(title = "Validator", line = 42, "Invalid value");
```
