# Utilities for implementing GitHub Actions

This crate provides basic utilities that help you develop [custom GitHub Actions](https://docs.github.com/en/actions/concepts/workflows-and-actions/custom-actions) in Rust.

## Environment Variables

```rust
use gha::github_workspace;
use std::fs::File;

let workspace = github_workspace();
let my_file = File::open(workspace.append("my_file.yaml"))?
```

## Messages

Easily generate [workflow commands](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-commands); e.g.,

```rust
use gha::{debug, error, mask, set_failed};

debug!("current dir: {:#?}", std::env::current_dir());
error!(title = "Validator", line = 42, "Invalid value");
mask!("{}", secret_token);
// set_failed! prints `::error::...` and exits with status 1
if !ok { set_failed!("aborting: {reason}"); }
```

## Environment Files

Append to the workflow's environment files (`$GITHUB_ENV`, `$GITHUB_OUTPUT`,
`$GITHUB_PATH`, `$GITHUB_STEP_SUMMARY`):

```rust
use gha::{
    GITHUB_ENV, GITHUB_PATH, GITHUB_STEP_SUMMARY,
    append_name_value, append_path, append_summary,
};
use std::env;
use std::fs::OpenOptions;

let mut env_file = OpenOptions::new().append(true).open(env::var(GITHUB_ENV)?)?;
append_name_value(&mut env_file, "VERSION", "1.2.3")?;

let mut path_file = OpenOptions::new().append(true).open(env::var(GITHUB_PATH)?)?;
append_path(&mut path_file, "/opt/my-tool/bin")?;

let mut summary = OpenOptions::new().append(true).open(env::var(GITHUB_STEP_SUMMARY)?)?;
append_summary(&mut summary, "## Build succeeded\n\nAll checks passed.")?;
```

## Project Template

To get started with [GitHub Actions in Rust](https://www.educative.io/blog/custom-github-actions-in-rust), use the following [project template](https://github.com/ecliptical/rust-gha-template):

```
cargo generate https://github.com/ecliptical/rust-gha-template
```
