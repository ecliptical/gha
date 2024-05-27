//! [Grouping log lines](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#grouping-log-lines)

/// Mark the start of the log line group.
pub fn start_group(title: &str) {
    println!("::group::{title}");
}

/// Mark the end of the log line group.
pub fn end_group() {
    println!("::endgroup::");
}

/// Perform arbitrary logic in the context of a log line group.
pub fn group<F: FnOnce()>(title: &str, f: F) {
    start_group(title);
    f();
    end_group();
}
