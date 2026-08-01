use anyhow::Result;
use clap::Subcommand;

mod build;
mod index;
mod init;
mod new;
mod resolve;
mod run;

/// Top-level Beru commands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create a new Beru C++ project
    New(new::NewArgs),

    /// Initialize Beru in an existing directory
    Init(init::InitArgs),

    /// Build the project
    Build(build::BuildArgs),

    /// Build and run the project (executables only)
    Run(run::RunArgs),

    /// Resolve dependencies and generate Beru.lock
    Resolve(resolve::ResolveArgs),

    /// Manage the package registry index
    Index(index::IndexArgs),
}

/// Dispatch a command.
pub fn run(cmd: Command) -> Result<()> {
    match cmd {
        Command::New(args) => new::exec(args),
        Command::Init(args) => init::exec(args),
        Command::Build(args) => build::exec(args),
        Command::Run(args) => run::exec(args),
        Command::Resolve(args) => resolve::exec(args),
        Command::Index(args) => index::exec(args),
    }
}
