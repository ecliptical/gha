use gha::{GITHUB_WORKSPACE, RUNNER_ARCH, RunnerArch, runner_arch, github_workspace, debug};
use std::env;

fn main() {
    env::set_var(GITHUB_WORKSPACE, "/tmp/test-ws");
    env::set_var(RUNNER_ARCH, RunnerArch::Arm64.to_string());
    
    debug!("workspace path: {:?}", github_workspace());
    debug!("runner arch: {:?}", runner_arch());
}
