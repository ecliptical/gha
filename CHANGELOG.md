# Changelog

All notable changes to this crate are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this crate
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.8]

### Added

- New env-file helpers:
  - `append_path` — append an entry to `$GITHUB_PATH`.
  - `append_summary` — append Markdown to `$GITHUB_STEP_SUMMARY`.
- New workflow-command macros:
  - `mask!` — emit `::add-mask::` to register a value as a secret.
  - `set_failed!` — emit an `::error::` message and exit with status 1.
  - `stop_commands!` / `resume_commands!` — pause and resume workflow
    command processing for blocks of literal output.

### Changed

- Refreshed all GitHub Actions documentation links to their current
  canonical URLs (the old `using-workflows/`, `creating-actions/`,
  `monitoring-and-troubleshooting-workflows/`, and
  `using-github-hosted-runners/` paths now redirect).

## [0.1.7]

Previous release. See git history for details.
