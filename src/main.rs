// SPDX-FileCopyrightText: 2026 The Vibe Killer contributers
// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-GAL
// SPDX-FileContributor: Jakob Schwanenberg
// SPDX-FileContributor: Corvidae Parrot <yetanotherparrot@posteo.de>

use clap::{Parser, Subcommand, ValueEnum};
use miette::{Diagnostic, Result};
use std::path::{Path, PathBuf};
use thiserror::Error;

mod dir_descriptions;
mod write_basic;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long, default_value_t = false)]
    #[cfg_attr(debug_assertions, arg(default_value_t = true))]
    dry: bool,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Init Vibe Killer in repository
    Init {
        #[clap(value_enum)]
        platforms: Vec<Platforms>,
        #[clap(short, long)]
        evil: bool,
    },
    /// Report on the status of Vibe Killer in the repository
    Status,
}

#[derive(ValueEnum, Debug, Clone, Subcommand)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
enum Platforms {
    Claude,
    Copilot,
    Cursor,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let plan: Vec<PlanStep> = match cli.command {
        None | Some(Commands::Status) => vec![PlanStep::ShowStatus],
        Some(Commands::Init { platforms: _, evil }) => vec![
            PlanStep::WriteBasic {
                dirs: gather_dirs()?,
                evil,
            },
            PlanStep::Symlink {
                from: "CLAUDE.md".into(),
                to: "AGENTS.md".into(),
            },
        ],
    };

    if cli.dry {
        println!("Dry run.");
    }

    for step in plan {
        if cli.dry {
            println!("{}", step.explain());
        } else {
            step.execute()?;
        }
    }

    Ok(())
}

#[derive(Debug, Error, Diagnostic)]
enum PlanningError {
    #[error("Could not read current directory.")]
    ReadCurrentDirectory(#[source] std::io::Error),
    #[error("Could not read a directory entry.")]
    ReadDirectoryEntry(#[source] std::io::Error),
}

#[derive(Debug, Clone)]
enum PlanStep {
    ShowStatus,
    Symlink {
        from: PathBuf,
        to: PathBuf,
    },
    WriteBasic {
        dirs: Vec<(String, String)>,
        evil: bool,
    },
}

impl PlanStep {
    fn explain(&self) -> String {
        match self {
            Self::ShowStatus => "Showing the status.".to_owned(),
            Self::Symlink { from, to } => format!(
                "Linking from {from} to {to}.",
                from = from.display(),
                to = to.display()
            ),
            Self::WriteBasic { dirs, evil } => {
                format!(
                    "Write the AGENTS.md with directories {} and evil mode {}.",
                    dirs.iter()
                        .map(|(f, _)| f.clone())
                        .collect::<Vec<String>>()
                        .join(", "),
                    if *evil { "enabled" } else { "disabled" }
                )
            }
        }
    }

    fn execute(&self) -> Result<(), ExecutionError> {
        match self {
            Self::ShowStatus => {
                show_status();
                Ok(())
            }
            Self::Symlink { from, to } => {
                #[cfg(target_family = "unix")]
                std::os::unix::fs::symlink(to, from).map_err(ExecutionError::SymlinkFailed)?;
                #[cfg(target_family = "windows")]
                std::os::windows::fs::symlink(to, from).map_err(ExecutionError::SymlinkFailed)?;
                Ok(())
            }
            Self::WriteBasic { dirs, evil } => write_basic::write_basic(dirs.clone(), *evil),
        }
    }
}

fn gather_dirs() -> Result<Vec<(String, String)>, PlanningError> {
    let entries = PathBuf::from(".")
        .read_dir()
        .map_err(PlanningError::ReadCurrentDirectory)?;
    let mut dirs = Vec::new();

    for entry in entries {
        let entry = entry.map_err(PlanningError::ReadDirectoryEntry)?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let Some(name) = path.file_name() else {
            continue;
        };

        let name = name.to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }

        dirs.push((name.clone(), dir_descriptions::get(&name)));
    }

    Ok(dirs)
}

fn show_status() {
    let agents_path = Path::new("AGENTS.md");
    let claude_path = Path::new("CLAUDE.md");
    let agents_exists = agents_path.exists();
    let claude_exists = claude_path.exists();

    println!(
        "AGENTS.md: {}",
        if agents_exists { "present" } else { "missing" }
    );
    println!(
        "CLAUDE.md: {}",
        if claude_exists { "present" } else { "missing" }
    );

    #[cfg(target_family = "unix")]
    match std::fs::read_link(claude_path) {
        Ok(target) => {
            let status = if target == agents_path {
                "correct symlink to AGENTS.md"
            } else {
                "symlink points elsewhere"
            };
            println!("CLAUDE.md link: {status}");
        }
        Err(_) => println!("CLAUDE.md link: not a symlink"),
    }
}

#[derive(Debug, Error, Diagnostic)]
enum ExecutionError {
    #[error("Could not create a symlink.")]
    SymlinkFailed(#[source] std::io::Error),
    #[error("Could not operate on AGENTS.md file.")]
    AgentsFileFailed(#[source] std::io::Error),
    #[error("Could not render the AGENTS.md template")]
    RenderHandlebars(#[source] handlebars::RenderError),
}
