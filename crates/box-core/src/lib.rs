//! Isolation engine for agentjail.
//!
//! Planned modules, one per milestone:
//! - `namespace`: user, PID, mount, IPC, UTS and network namespaces (M1, M4)
//! - `rootfs`: read-only bind mounts and `pivot_root` (M1)
//! - `cgroup`: cgroups v2 limits for memory, CPU and process count (M2)
//! - `seccomp`: syscall filter and Landlock rules (M2)
//! - `snapshot`: OverlayFS snapshot, rollback, diff and commit (M3)

/// Settings for one sandbox run.
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// Host directory mounted read-write inside the sandbox.
    pub workspace: std::path::PathBuf,
    /// Program and arguments to run inside the sandbox.
    pub command: Vec<String>,
}

/// Errors returned by the isolation engine.
#[derive(Debug)]
pub enum Error {
    /// The requested feature is not built yet.
    NotImplemented(&'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotImplemented(what) => write!(f, "not implemented yet: {what}"),
        }
    }
}

impl std::error::Error for Error {}

/// Runs `config.command` inside a new sandbox and returns its exit code.
pub fn run(config: &SandboxConfig) -> Result<i32, Error> {
    let _ = config;
    Err(Error::NotImplemented("sandbox run (milestone 1)"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_is_not_implemented_yet() {
        let config = SandboxConfig {
            workspace: ".".into(),
            command: vec!["true".into()],
        };
        assert!(matches!(run(&config), Err(Error::NotImplemented(_))));
    }
}
