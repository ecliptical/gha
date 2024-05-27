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
pub fn group<T, F: FnOnce() -> T>(title: &str, f: F) -> T {
    start_group(title);
    let retval = f();
    end_group();
    retval
}
