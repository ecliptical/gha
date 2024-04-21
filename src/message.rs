//! Macros for [workflow commands](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions).

/// Outputs a [debug message](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#setting-a-debug-message).
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        for line in format!($($arg)*).lines() {
            println!("::debug::{line}");
        }
    };
}

/// Outputs a [notice message](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#setting-a-notice-message).
#[macro_export]
macro_rules! notice {
    (title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, col = $col:tt, end_col = $end_col:tt, $($arg:tt)*) => {
        $crate::message!(
            "notice",
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line,
            col = $col,
            end_col = $end_col,
            $($arg)*
        );
    };

    (title = $title:tt, file = $file:tt, line = $line:tt, col = $col:tt, end_col = $end_col:tt, $($arg:tt)*) => {
        $crate::message!(
            "notice",
            title = $title,
            file = $file,
            line = $line,
            col = $col,
            end_col = $end_col,
            $($arg)*
        );
    };

    (title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, col = $col:tt, $($arg:tt)*) => {
        $crate::message!(
            "notice",
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line,
            col = $col,
            $($arg)*
        );
    };

    (title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, $($arg:tt)*) => {
        $crate::message!(
            "notice",
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line,
            $($arg)*
        );
    };

    (title = $title:tt, file = $file:tt, line = $line:tt, col = $col:tt, $($arg:tt)*) => {
        $crate::message!(
            "notice",
            title = $title,
            file = $file,
            line = $line,
            col = $col,
            $($arg)*
        );
    };

    (title = $title:tt, file = $file:tt, line = $line:tt, $($arg:tt)*) => {
        $crate::message!(
            "notice",
            title = $title,
            file = $file,
            line = $line,
            $($arg)*
        );
    };

    (title = $title:tt, file = $file:tt, $($arg:tt)*) => {
        $crate::message!("notice", title = $title, file = $file, $($arg)*);
    };

    (title = $title:tt, $($arg:tt)*) => {
        $crate::message!("notice", title = $title, $($arg)*);
    };
}

/// Outputs a [warning message](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#setting-a-warning-message).
#[macro_export]
macro_rules! warning {
    (title = $title:tt, $($arg:tt)*) => {
        $crate::message!("warning", title = $title, $($arg)*);
    };

    (title = $title:tt, file = $file:tt, $($arg:tt)*) => {
        $crate::message!("warning", title = $title, file = $file, $($arg)*);
    };

    (title = $title:tt, file = $file:tt, line = $line:tt, $($arg:tt)*) => {
        $crate::message!(
            "warning",
            title = $title,
            file = $file,
            line = $line,
            $($arg)*
        );
    };

    (title = $title:tt, file = $file:tt, line = $line:tt, col = $col:tt, $($arg:tt)*) => {
        $crate::message!(
            "warning",
            title = $title,
            file = $file,
            line = $line,
            col = $col,
            $($arg)*
        );
    };

    (title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, $($arg:tt)*) => {
        $crate::message!(
            "warning",
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line,
            $($arg)*
        );
    };

    (title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, col = $col:tt, $($arg:tt)*) => {
        $crate::message!(
            "warning",
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line,
            col = $col,
            $($arg)*
        );
    };

    (title = $title:tt, file = $file:tt, line = $line:tt, col = $col:tt, end_col = $end_col:tt, $($arg:tt)*) => {
        $crate::message!(
            "warning",
            title = $title,
            file = $file,
            line = $line,
            col = $col,
            end_col = $end_col,
            $($arg)*
        );
    };

    (title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, col = $col:tt, end_col = $end_col:tt, $($arg:tt)*) => {
        $crate::message!(
            "warning",
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line,
            col = $col,
            end_col = $end_col,
            $($arg)*
        );
    };
}

/// Outputs an [error message](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#setting-an-error-message).
#[macro_export]
macro_rules! error {
    (title = $title:tt, $($arg:tt)*) => {
        $crate::message!("error", title = $title, $($arg)*);
    };

    (title = $title:tt, file = $file:tt, $($arg:tt)*) => {
        $crate::message!("error", title = $title, file = $file, $($arg)*);
    };

    (title = $title:tt, file = $file:tt, line = $line:tt, $($arg:tt)*) => {
        $crate::message!(
            "error",
            title = $title,
            file = $file,
            line = $line,
            $($arg)*
        );
    };

    (title = $title:tt, file = $file:tt, line = $line:tt, col = $col:tt, $($arg:tt)*) => {
        $crate::message!(
            "error",
            title = $title,
            file = $file,
            line = $line,
            col = $col,
            $($arg)*
        );
    };

    (title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, $($arg:tt)*) => {
        $crate::message!(
            "error",
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line,
            $($arg)*
        );
    };

    (title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, col = $col:tt, $($arg:tt)*) => {
        $crate::message!(
            "error",
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line,
            col = $col,
            $($arg)*
        );
    };

    (title = $title:tt, file = $file:tt, line = $line:tt, col = $col:tt, end_col = $end_col:tt, $($arg:tt)*) => {
        $crate::message!(
            "error",
            title = $title,
            file = $file,
            line = $line,
            col = $col,
            end_col = $end_col,
            $($arg)*
        );
    };

    (title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, col = $col:tt, end_col = $end_col:tt, $($arg:tt)*) => {
        $crate::message!(
            "error",
            title = $title,
            file = $file,
            line = $line,
            end_line = $end_line,
            col = $col,
            end_col = $end_col,
            $($arg)*
        );
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! message {
    ($type:expr, title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, col = $col:tt, end_col = $end_col:tt, $($arg:tt)*) => {
        println!(
            "::{} title={},file={},line={},endLine={},col={},endColumn={}::{}",
            $type, $title, $file, $line, $end_line, $col, $end_col, format!($($arg)*).replace('\n', " ")
        );
    };

    ($type:expr, title = $title:tt, file = $file:tt, line = $line:tt, col = $col:tt, end_col = $end_col:tt, $($arg:tt)*) => {
        println!(
            "::{} title={},file={},line={},col={},endColumn={}::{}",
            $type, $title, $file, $line, $col, $end_col, format!($($arg)*).replace('\n', " ")
        );
    };

    ($type:expr, title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, col = $col:tt, $($arg:tt)*) => {
        println!(
            "::{} title={},file={},line={},endLine={},col={}::{}",
            $type, $title, $file, $line, $end_line, $col, format!($($arg)*).replace('\n', " ")
        );
    };
    ($type:expr, title = $title:tt, file = $file:tt, line = $line:tt, end_line = $end_line:tt, $($arg:tt)*) => {
        println!(
            "::{} title={},file={},line={},endLine={}::{}",
            $type, $title, $file, $line, $end_line, format!($($arg)*).replace('\n', " ")
        );
    };

    ($type:expr, title = $title:tt, file = $file:tt, line = $line:tt, col = $col:tt, $($arg:tt)*) => {
        println!(
            "::{} title={},file={},line={},col={}::{}",
            $type, $title, $file, $line, $col, format!($($arg)*).replace('\n', " ")
        );
    };

    ($type:expr, title = $title:tt, file = $file:tt, line = $line:tt, $($arg:tt)*) => {
        println!(
            "::{} title={},file={},line={}::{}",
            $type, $title, $file, $line, format!($($arg)*).replace('\n', " ")
        );
    };

    ($type:expr, title = $title:tt, file = $file:tt, $($arg:tt)*) => {
        println!("::{} title={},file={}::{}", $type, $title, $file, format!($($arg)*).replace('\n', " "));
    };

    ($type:expr, title = $title:tt, $($arg:tt)*) => {
        println!("::{} title={}::{}", $type, $title, format!($($arg)*).replace('\n', " "));
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
        notice!(title = "test title", "test msg");
    }

    #[test]
    fn notice_file() {
        notice!(title = "test title", file = "test file", "test msg");
    }

    #[test]
    fn notice_line() {
        notice!(
            title = "test title",
            file = "test file",
            line = 2,
            "test msg"
        );
    }

    #[test]
    fn notice_end_line() {
        notice!(
            title = "test title",
            file = "test file",
            line = 2,
            end_line = 3,
            "test msg"
        );
    }

    #[test]
    fn notice_end_line_col() {
        notice!(
            title = "test title",
            file = "test file",
            line = 2,
            end_line = 3,
            col = 1,
            "test msg"
        );
    }

    #[test]
    fn notice_col() {
        notice!(
            title = "test title",
            file = "test file",
            line = 2,
            col = 1,
            "test msg"
        );
    }

    #[test]
    fn notice_end_col() {
        notice!(
            title = "test title",
            file = "test file",
            line = 2,
            col = 1,
            end_col = 9,
            "test msg"
        );
    }

    #[test]
    fn notice_full() {
        notice!(
            title = "test title",
            file = "test file",
            line = 2,
            end_line = 3,
            col = 1,
            end_col = 9,
            "test msg: {}",
            1
        );
    }

    #[test]
    fn warning_msg() {
        warning!(title = "test title", "test msg");
    }

    #[test]
    fn warning_file() {
        warning!(title = "test title", file = "test file", "test msg");
    }

    #[test]
    fn warning_line() {
        warning!(
            title = "test title",
            file = "test file",
            line = 2,
            "test msg"
        );
    }

    #[test]
    fn warning_end_line() {
        warning!(
            title = "test title",
            file = "test file",
            line = 2,
            end_line = 3,
            "test msg"
        );
    }

    #[test]
    fn warning_end_line_col() {
        warning!(
            title = "test title",
            file = "test file",
            line = 2,
            end_line = 3,
            col = 1,
            "test msg"
        );
    }

    #[test]
    fn warning_col() {
        warning!(
            title = "test title",
            file = "test file",
            line = 2,
            col = 1,
            "test msg"
        );
    }

    #[test]
    fn warning_end_col() {
        warning!(
            title = "test title",
            file = "test file",
            line = 2,
            col = 1,
            end_col = 9,
            "test msg"
        );
    }

    #[test]
    fn warning_full() {
        warning!(
            title = "test title",
            file = "test file",
            line = 2,
            end_line = 3,
            col = 1,
            end_col = 9,
            "test msg: {}",
            1
        );
    }

    #[test]
    fn error_msg() {
        error!(title = "test title", "test msg");
    }

    #[test]
    fn error_file() {
        error!(title = "test title", file = "test file", "test msg");
    }

    #[test]
    fn error_line() {
        error!(
            title = "test title",
            file = "test file",
            line = 2,
            "test msg"
        );
    }

    #[test]
    fn error_end_line() {
        error!(
            title = "test title",
            file = "test file",
            line = 2,
            end_line = 3,
            "test msg"
        );
    }

    #[test]
    fn error_end_line_col() {
        error!(
            title = "test title",
            file = "test file",
            line = 2,
            end_line = 3,
            col = 1,
            "test msg"
        );
    }

    #[test]
    fn error_col() {
        error!(
            title = "test title",
            file = "test file",
            line = 2,
            col = 1,
            "test msg"
        );
    }

    #[test]
    fn error_end_col() {
        error!(
            title = "test title",
            file = "test file",
            line = 2,
            col = 1,
            end_col = 9,
            "test msg"
        );
    }

    #[test]
    fn error_full() {
        error!(
            title = "test title",
            file = "test file",
            line = 2,
            end_line = 3,
            col = 1,
            end_col = 9,
            "test msg {}",
            1
        );
    }
}
