//! Functions for writing formatted values to action-specific files.

use std::fmt::Display;
use std::io::{Result, Write};

/// Appends a `<NAME>=<VALUE>` line to the given file.
/// Suitable for writing to [crate::env::GITHUB_ENV], [crate::env::GITHUB_OUTPUT], and [crate::env::GITHUB_STATE] files.
/// The value must not contain the `\n` character.
pub fn append_name_value<F: Write, K: Display, V: Display>(
    file: &mut F,
    name: K,
    value: V,
) -> Result<()> {
    writeln!(file, "{name}={value}")
}

/// Appends a properly formatted multi-line version of a `<NAME>=<VALUE>` pair.
/// Suitable for writing to the [crate::env::GITHUB_ENV] file.
/// The supplied delimiter must not occur as a single line anywhere in the value string,
/// and must not contain the `\n` character.
pub fn append_name_value_ml<F: Write, K: Display, V: Display, D: Display>(
    file: &mut F,
    name: K,
    value: V,
    delim: D,
) -> Result<()> {
    writeln!(file, "{name}<<{delim}")?;
    writeln!(file, "{value}")?;
    writeln!(file, "{delim}")
}

/// Appends a single path entry to the given file, prepending it to the system `PATH`.
/// Suitable for writing to the [crate::env::GITHUB_PATH] file.
/// The path must not contain the `\n` character.
pub fn append_path<F: Write, P: Display>(file: &mut F, path: P) -> Result<()> {
    writeln!(file, "{path}")
}

/// Appends a chunk of GitHub-flavored Markdown to the given file.
/// Suitable for writing to the [crate::env::GITHUB_STEP_SUMMARY] file.
///
/// Each step's job summary is limited to 1 MiB of Markdown and at most
/// 20 step summaries are displayed per job. After a step has completed,
/// uploaded summaries cannot be modified by subsequent steps.
pub fn append_summary<F: Write, M: Display>(file: &mut F, markdown: M) -> Result<()> {
    writeln!(file, "{markdown}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_line() {
        let mut buf = Vec::default();
        let res = append_name_value(&mut buf, "test_key", "test_value");
        assert!(res.is_ok());
        assert_eq!(String::from_utf8(buf).unwrap(), "test_key=test_value\n");
    }

    #[test]
    fn append_multiple_lines() {
        let mut buf = Vec::default();
        let res = append_name_value_ml(&mut buf, "test_key", "test\nvalue", "EOF");
        assert!(res.is_ok());
        assert_eq!(
            String::from_utf8(buf).unwrap(),
            "test_key<<EOF\ntest\nvalue\nEOF\n"
        );
    }

    #[test]
    fn append_path_line() {
        let mut buf = Vec::default();
        let res = append_path(&mut buf, "/home/runner/.local/bin");
        assert!(res.is_ok());
        assert_eq!(String::from_utf8(buf).unwrap(), "/home/runner/.local/bin\n");
    }

    #[test]
    fn append_summary_markdown() {
        let mut buf = Vec::default();
        let res = append_summary(&mut buf, "# Hello\n\n- item 1\n- item 2");
        assert!(res.is_ok());
        assert_eq!(
            String::from_utf8(buf).unwrap(),
            "# Hello\n\n- item 1\n- item 2\n"
        );
    }
}
