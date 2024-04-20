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
}
