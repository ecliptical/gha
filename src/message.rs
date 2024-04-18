//! Macros for [workflow commands](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions).

/// Outputs a [debug message](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#setting-a-debug-message).
#[macro_export]
macro_rules! debug {
    ($message:expr) => {
        println!("::debug::{}", $message);
    };
}

/// Outputs a [notice message](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#setting-a-notice-message).
#[macro_export]
macro_rules! notice {
    ($message:expr, title = $title:tt) => {
        $crate::message!("notice", $message, title = $title);
    };

    ($message:expr, title = $title:tt, file = $file:tt) => {
        $crate::message!("notice", $message, title = $title, file = $file);
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt) => {
        $crate::message!(
            "notice",
            $message,
            title = $title,
            file = $file,
            line = $line
        );
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt, col = $col:tt) => {
        $crate::message!(
            "notice",
            $message,
            title = $title,
            file = $file,
            line = $line,
            col = $col
        );
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt) => {
        $crate::message!(
            "notice",
            $message,
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line
        );
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, col = $col:tt) => {
        $crate::message!(
            "notice",
            $message,
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line,
            col = $col
        );
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt, col = $col:tt, end_col = $end_col:tt) => {
        $crate::message!(
            "notice",
            $message,
            title = $title,
            file = $file,
            line = $line,
            col = $col,
            end_col = $end_col
        );
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, col = $col:tt, end_col = $end_col:tt) => {
        $crate::message!(
            "notice",
            $message,
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line,
            col = $col,
            end_col = $end_col
        );
    };
}

/// Outputs a [warning message](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#setting-a-warning-message).
#[macro_export]
macro_rules! warning {
    ($message:expr, title = $title:tt) => {
        $crate::message!("warning", $message, title = $title);
    };

    ($message:expr, title = $title:tt, file = $file:tt) => {
        $crate::message!("warning", $message, title = $title, file = $file);
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt) => {
        $crate::message!(
            "warning",
            $message,
            title = $title,
            file = $file,
            line = $line
        );
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt, col = $col:tt) => {
        $crate::message!(
            "warning",
            $message,
            title = $title,
            file = $file,
            line = $line,
            col = $col
        );
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt) => {
        $crate::message!(
            "warning",
            $message,
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line
        );
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, col = $col:tt) => {
        $crate::message!(
            "warning",
            $message,
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line,
            col = $col
        );
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt, col = $col:tt, end_col = $end_col:tt) => {
        $crate::message!(
            "warning",
            $message,
            title = $title,
            file = $file,
            line = $line,
            col = $col,
            end_col = $end_col
        );
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, col = $col:tt, end_col = $end_col:tt) => {
        $crate::message!(
            "warning",
            $message,
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line,
            col = $col,
            end_col = $end_col
        );
    };
}

/// Outputs an [error message](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#setting-an-error-message).
#[macro_export]
macro_rules! error {
    ($message:expr, title = $title:tt) => {
        $crate::message!("error", $message, title = $title);
    };

    ($message:expr, title = $title:tt, file = $file:tt) => {
        $crate::message!("error", $message, title = $title, file = $file);
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt) => {
        $crate::message!(
            "error",
            $message,
            title = $title,
            file = $file,
            line = $line
        );
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt, col = $col:tt) => {
        $crate::message!(
            "error",
            $message,
            title = $title,
            file = $file,
            line = $line,
            col = $col
        );
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt) => {
        $crate::message!(
            "error",
            $message,
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line
        );
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, col = $col:tt) => {
        $crate::message!(
            "error",
            $message,
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line,
            col = $col
        );
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt, col = $col:tt, end_col = $end_col:tt) => {
        $crate::message!(
            "error",
            $message,
            title = $title,
            file = $file,
            line = $line,
            col = $col,
            end_col = $end_col
        );
    };

    ($message:expr, title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, col = $col:tt, end_col = $end_col:tt) => {
        $crate::message!(
            "error",
            $message,
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line,
            col = $col,
            end_col = $end_col
        );
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! message {
    ($type:expr, $message:expr, title = $title:tt) => {
        println!("::{} title={}::{}", $type, $title, $message);
    };

    ($type:expr, $message:expr, title = $title:tt, file = $file:tt) => {
        println!("::{} title={},file={}::{}", $type, $title, $file, $message);
    };

    ($type:expr, $message:expr, title = $title:tt, file = $file:tt, line = $line:tt) => {
        println!(
            "::{} title={},file={},line={}::{}",
            $type, $title, $file, $line, $message
        );
    };

    ($type:expr, $message:expr, title = $title:tt, file = $file:tt, line = $line:tt, col = $col:tt) => {
        println!(
            "::{} title={},file={},line={},col={}::{}",
            $type, $title, $file, $line, $col, $message
        );
    };

    ($type:expr, $message:expr, title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt) => {
        println!(
            "::{} title={},file={},line={},endLine={}::{}",
            $type, $title, $file, $line, $end_line, $message
        );
    };

    ($type:expr, $message:expr, title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, col = $col:tt) => {
        println!(
            "::{} title={},file={},line={},endLine={},col={}::{}",
            $type, $title, $file, $line, $end_line, $col, $message
        );
    };

    ($type:expr, $message:expr, title = $title:tt, file = $file:tt, line = $line:tt, col = $col:tt, end_col = $end_col:tt) => {
        println!(
            "::{} title={},file={},line={},col={},endColumn={}::{}",
            $type, $title, $file, $line, $col, $end_col, $message
        );
    };

    ($type:expr, $message:expr, title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, col = $col:tt, end_col = $end_col:tt) => {
        println!(
            "::{} title={},file={},line={},endLine={},col={},endColumn={}::{}",
            $type, $title, $file, $line, $end_line, $col, $end_col, $message
        );
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn debug_msg() {
        debug!("test");
    }

    #[test]
    fn notice_msg() {
        notice!("test msg", title = "test title");
    }

    #[test]
    fn notice_file() {
        notice!("test msg", title = "test title", file = "test file");
    }

    #[test]
    fn notice_line() {
        notice!(
            "test msg",
            title = "test title",
            file = "test file",
            line = 2
        );
    }

    #[test]
    fn notice_end_line() {
        notice!(
            "test msg",
            title = "test title",
            file = "test file",
            line = 2,
            end_line = 3
        );
    }

    #[test]
    fn notice_col() {
        notice!(
            "test msg",
            title = "test title",
            file = "test file",
            line = 2,
            col = 1
        );
    }

    #[test]
    fn notice_end_col() {
        notice!(
            "test msg",
            title = "test title",
            file = "test file",
            line = 2,
            col = 1,
            end_col = 9
        );
    }

    #[test]
    fn notice_full() {
        notice!(
            "test msg",
            title = "test title",
            file = "test file",
            line = 2,
            end_line = 3,
            col = 1,
            end_col = 9
        );
    }

    #[test]
    fn warning_msg() {
        warning!("test msg", title = "test title");
    }

    #[test]
    fn warning_file() {
        warning!("test msg", title = "test title", file = "test file");
    }

    #[test]
    fn warning_line() {
        warning!(
            "test msg",
            title = "test title",
            file = "test file",
            line = 2
        );
    }

    #[test]
    fn warning_end_line() {
        warning!(
            "test msg",
            title = "test title",
            file = "test file",
            line = 2,
            end_line = 3
        );
    }

    #[test]
    fn warning_col() {
        warning!(
            "test msg",
            title = "test title",
            file = "test file",
            line = 2,
            col = 1
        );
    }

    #[test]
    fn warning_end_col() {
        warning!(
            "test msg",
            title = "test title",
            file = "test file",
            line = 2,
            col = 1,
            end_col = 9
        );
    }

    #[test]
    fn warning_full() {
        warning!(
            "test msg",
            title = "test title",
            file = "test file",
            line = 2,
            end_line = 3,
            col = 1,
            end_col = 9
        );
    }

    #[test]
    fn error_msg() {
        error!("test msg", title = "test title");
    }

    #[test]
    fn error_file() {
        error!("test msg", title = "test title", file = "test file");
    }

    #[test]
    fn error_line() {
        error!(
            "test msg",
            title = "test title",
            file = "test file",
            line = 2
        );
    }

    #[test]
    fn error_end_line() {
        error!(
            "test msg",
            title = "test title",
            file = "test file",
            line = 2,
            end_line = 3
        );
    }

    #[test]
    fn error_col() {
        error!(
            "test msg",
            title = "test title",
            file = "test file",
            line = 2,
            col = 1
        );
    }

    #[test]
    fn error_end_col() {
        error!(
            "test msg",
            title = "test title",
            file = "test file",
            line = 2,
            col = 1,
            end_col = 9
        );
    }

    #[test]
    fn error_full() {
        error!(
            "test msg",
            title = "test title",
            file = "test file",
            line = 2,
            end_line = 3,
            col = 1,
            end_col = 9
        );
    }
}
