// SPDX-FileCopyrightText: 2026 The Vibe Killer contributers
// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-GAL
// SPDX-FileContributor: Jakob Schwanenberg
// SPDX-FileContributor: Corvidae Parrot <yetanotherparrot@posteo.de>

use clap::{Parser, Subcommand};
use miette::{Diagnostic, Result};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Init Vibe Killer in repository
    Init,
    /// Report on the status of Vibe Killer in the repository
    Status,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let plan: Vec<PlanStep> = match cli.command {
        None | Some(Commands::Status) => vec![PlanStep::ShowStatus],
        Some(Commands::Init) => return Err(PlanningError::NotImplemented.into()),
    };

    for step in plan {
        step.execute()?;
    }

    Ok(())
}

#[derive(Debug, Error, Diagnostic)]
enum PlanningError {
    #[error("This is not implemented yet. Sorry! 👉👈")]
    NotImplemented,
}

#[derive(Debug, Clone)]
enum PlanStep {
    ShowStatus,
    _Symlink { from: PathBuf, to: PathBuf },
}

impl PlanStep {
    fn execute(&self) -> Result<(), ExecutionError> {
        match self {
            Self::ShowStatus => Err(ExecutionError::NotImplemented),
            Self::_Symlink { from, to } => {
                #[cfg(target_family = "unix")]
                std::os::unix::fs::symlink(to, from).map_err(ExecutionError::SymlinkFailed)?;
                #[cfg(target_family = "windows")]
                std::os::windows::fs::symlink(to, from).map_err(ExecutionError::SymlinkFailed)?;
                Ok(())
            }
        }
    }
}

#[derive(Debug, Error, Diagnostic)]
enum ExecutionError {
    #[error("This is not implemented yet. Sorry! 👉👈")]
    NotImplemented,
    #[error("Could not create a symlink.")]
    SymlinkFailed(#[source] std::io::Error),
}
