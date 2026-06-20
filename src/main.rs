// SPDX-FileCopyrightText: 2026 The Vibe Killer contributers
// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-GAL
// SPDX-FileContributor: Jakob Schwanenberg

use clap::{Parser, Subcommand};
use miette::{Diagnostic, Result};
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
}

impl PlanStep {
    const fn execute(&self) -> Result<(), ExecutionError> {
        match self {
            Self::ShowStatus => Err(ExecutionError::NotImplemented),
        }
    }
}

#[derive(Debug, Error, Diagnostic)]
enum ExecutionError {
    #[error("This is not implemented yet. Sorry! 👉👈")]
    NotImplemented,
}
