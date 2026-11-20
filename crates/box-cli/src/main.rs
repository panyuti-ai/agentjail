use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

/// Run untrusted AI agent code in an isolated, rollback-able sandbox.
#[derive(Parser)]
#[command(name = "agentjail", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run a command inside a new sandbox.
    Run {
        /// Directory mounted read-write inside the sandbox.
        #[arg(long, default_value = ".")]
        workspace: PathBuf,
        /// Command to run, given after `--`.
        #[arg(last = true, required = true)]
        command: Vec<String>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Command::Run { workspace, command } => {
            let config = box_core::SandboxConfig { workspace, command };
            match box_core::run(&config) {
                Ok(code) => ExitCode::from(u8::try_from(code).unwrap_or(1)),
                Err(err) => {
                    eprintln!("agentjail: {err}");
                    ExitCode::FAILURE
                }
            }
        }
    }
}
