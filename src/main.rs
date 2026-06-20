// SPDX-FileCopyrightText: 2026 The Vibe Killer contributers
// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-GAL
// SPDX-FileContributor: Jakob Schwanenberg
// SPDX-FileContributor: Corvidae Parrot <yetanotherparrot@posteo.de>

use clap::{Parser, Subcommand};
use miette::{Diagnostic, Result};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use thiserror::Error;

mod write_basic;

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
        Some(Commands::Init) => vec![
            PlanStep::WriteBasic,
            PlanStep::Symlink {
                from: "CLAUDE.md".into(),
                to: "AGENTS.md".into(),
            },
        ],
    };

    for step in plan {
        step.execute()?;
    }

    Ok(())
}

#[derive(Debug, Error, Diagnostic)]
enum _PlanningError {
    #[error("This is not implemented yet. Sorry! 👉👈")]
    NotImplemented,
}

#[derive(Debug, Clone)]
enum PlanStep {
    ShowStatus,
    Symlink { from: PathBuf, to: PathBuf },
    WriteBasic,
}

impl PlanStep {
    fn execute(&self) -> Result<(), ExecutionError> {
        match self {
            Self::ShowStatus => Err(ExecutionError::NotImplemented),
            Self::Symlink { from, to } => {
                #[cfg(target_family = "unix")]
                std::os::unix::fs::symlink(to, from).map_err(ExecutionError::SymlinkFailed)?;
                #[cfg(target_family = "windows")]
                std::os::windows::fs::symlink(to, from).map_err(ExecutionError::SymlinkFailed)?;
                Ok(())
            }
            Self::WriteBasic => write_basic::write_basic(),
        }
    }
}

#[derive(Debug, Error, Diagnostic)]
enum ExecutionError {
    #[error("This is not implemented yet. Sorry! 👉👈")]
    NotImplemented,
    #[error("Could not create a symlink.")]
    SymlinkFailed(#[source] std::io::Error),
    #[error("Could not operate on AGENTS.md file.")]
    AgentsFileFailed(#[source] std::io::Error),
}
