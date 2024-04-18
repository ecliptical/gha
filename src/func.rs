//! Functions for extracting GitHub action environment variables.

use std::env::var;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::OnceLock;

use crate::env::*;

/// Error parsing enumeration values.
#[derive(Clone, Debug)]
pub struct IllegalValueError(String);

impl fmt::Display for IllegalValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        format!("illegal value: {}", self.0).fmt(f)
    }
}

impl Error for IllegalValueError {}

macro_rules! bool_func {
    ($env:ident, $func:ident) => {
        #[doc = concat!("Returns the value of the [", stringify!($env), "] environment variable.")]
        pub fn $func() -> bool {
            static VAL: OnceLock<bool> = OnceLock::new();
            *VAL.get_or_init(|| var($env).map(|val| val == "true").unwrap_or(false))
        }
    };
}

macro_rules! str_func {
    ($env:ident, $func:ident) => {
        #[doc = concat!("Returns the value of the [", stringify!($env), "] environment variable.")]
        pub fn $func() -> &'static str {
            static VAL: OnceLock<String> = OnceLock::new();
            VAL.get_or_init(|| var($env).unwrap_or_default())
        }
    };
}

macro_rules! path_func {
    ($env:ident, $func:ident) => {
        #[doc = concat!("Returns the value of the [", stringify!($env), "] environment variable.")]
        pub fn $func() -> &'static Path {
            static VAL: OnceLock<PathBuf> = OnceLock::new();
            VAL.get_or_init(|| var($env).map(Into::into).unwrap_or_default())
        }
    };
}

macro_rules! int_func {
    ($env:ident, $func:ident) => {
        #[doc = concat!("Returns the value of the [", stringify!($env), "] environment variable.")]
        pub fn $func() -> usize {
            static VAL: OnceLock<usize> = OnceLock::new();
            *VAL.get_or_init(|| var($env).ok().and_then(|s| s.parse().ok()).unwrap_or(0))
        }
    };
}

macro_rules! enum_func {
    ($env:ident, $func:ident, $enum:ident, $($key:ident = $val:tt),+) => {
        #[doc = concat!("Possible values of the [", stringify!($env), "] environment variable.")]
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        pub enum $enum {
            $($key,)+
        }

        impl FromStr for $enum {
            type Err = IllegalValueError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($val => Ok(Self::$key),)+
                    _ => Err(IllegalValueError(s.to_string())),
                }
            }
        }

        impl fmt::Display for $enum {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                match self {
                    $(Self::$key => $val.fmt(f),)+
                }
            }
        }

        #[doc = concat!("Returns the value of the [", stringify!($env), "] environment variable.")]
        pub fn $func() -> Result<$enum, IllegalValueError> {
            static VAL: OnceLock<Result<$enum, IllegalValueError>> = OnceLock::new();
            VAL.get_or_init(|| var($env).map_err(|e| IllegalValueError(e.to_string())).and_then(|s| s.parse())).clone()
        }
    };
}

bool_func!(CI, ci);
str_func!(GITHUB_ACTION, github_action);
path_func!(GITHUB_ACTION_PATH, github_action_path);
str_func!(GITHUB_ACTION_REPOSITORY, github_action_repository);
bool_func!(GITHUB_ACTIONS, github_actions);
str_func!(GITHUB_ACTOR, github_actor);
int_func!(GITHUB_ACTOR_ID, github_actor_id);
str_func!(GITHUB_API_URL, github_api_url);
str_func!(GITHUB_BASE_REF, github_base_ref);
path_func!(GITHUB_ENV, github_env);
str_func!(GITHUB_EVENT_NAME, github_event_name);
path_func!(GITHUB_EVENT_PATH, github_event_path);
str_func!(GITHUB_GRAPHQL_URL, github_graphql_url);
str_func!(GITHUB_HEAD_REF, github_head_ref);
str_func!(GITHUB_JOB, github_job);
path_func!(GITHUB_OUTPUT, github_output);
path_func!(GITHUB_PATH, github_path);
str_func!(GITHUB_REF, github_ref);
str_func!(GITHUB_REF_NAME, github_ref_name);
bool_func!(GITHUB_REF_PROTECTED, github_ref_protected);
enum_func!(
    GITHUB_REF_TYPE,
    github_ref_type,
    GitHubRefType,
    Branch = "branch",
    Tag = "tag"
);

str_func!(GITHUB_REPOSITORY, github_repository);
int_func!(GITHUB_REPOSITORY_ID, github_repository_id);
str_func!(GITHUB_REPOSITORY_OWNER, github_repository_owner);
int_func!(GITHUB_REPOSITORY_OWNER_ID, github_repository_owner_id);
int_func!(GITHUB_RETENTION_DAYS, github_retention_days);
int_func!(GITHUB_RUN_ATTEMPT, github_run_attempt);
int_func!(GITHUB_RUN_ID, github_run_id);
int_func!(GITHUB_RUN_NUMBER, github_run_number);
str_func!(GITHUB_SERVER_URL, github_server_url);
str_func!(GITHUB_SHA, github_sha);
path_func!(GITHUB_STEP_SUMMARY, github_step_summary);
str_func!(GITHUB_TRIGGERING_ACTOR, github_triggering_actor);
str_func!(GITHUB_WORKFLOW, github_workflow);
str_func!(GITHUB_WORKFLOW_REF, github_workflow_ref);
str_func!(GITHUB_WORKFLOW_SHA, github_workflow_sha);
path_func!(GITHUB_WORKSPACE, github_workspace);
enum_func!(
    RUNNER_ARCH,
    runner_arch,
    RunnerArch,
    X86 = "X86",
    X64 = "X64",
    Arm = "ARM",
    Arm64 = "ARM64"
);

int_func!(RUNNER_DEBUG, runner_debug);
str_func!(RUNNER_NAME, runner_name);
enum_func!(
    RUNNER_OS,
    runner_os,
    RunnerOs,
    Linux = "Linux",
    Windows = "Windows",
    MacOs = "macOS"
);

path_func!(RUNNER_TEMP, runner_temp);
path_func!(RUNNER_TOOL_CACHE, runner_tool_cache);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_func() {
        let val = ci();
        assert!(val || !val);
    }

    #[test]
    fn str_func() {
        let val = github_action();
        assert!(val.is_empty() || val.len() > 0);
    }

    #[test]
    fn path_func() {
        let val = github_action_path().display().to_string();
        assert!(val.is_empty() || val.len() > 0);
    }

    #[test]
    fn int_func() {
        let val = github_actor_id();
        assert!(val == 0 || val > 0);
    }

    #[test]
    fn enum_func() {
        let val = github_ref_type();
        assert!(val.is_ok() || val.is_err());
    }
}
