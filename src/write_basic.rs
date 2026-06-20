// SPDX-FileCopyrightText: 2026 The Vibe Killer contributers
// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-GAL
// SPDX-FileContributor: Jakob Schwanenberg
// SPDX-FileContributor: Corvidae Parrot <yetanotherparrot@posteo.de>

use super::ExecutionError;
use handlebars::Handlebars;
use serde::Serialize;
use std::fs::File;
use std::io::Write;

const AGENTS_MD_TEMPLATE: &str = include_str!("AGENTS.md");
const AGENTS_EVIL_TEMPLATE: &str = include_str!("AGENTS_EVIL.md");

pub fn write_basic(dirs: Vec<(String, String)>, evil: bool) -> Result<(), ExecutionError> {
    let reg = Handlebars::new();
    let mut file = File::create("AGENTS.md").map_err(ExecutionError::AgentsFileFailed)?;
    file.write_all(
        reg.render_template(
            if evil {
                AGENTS_EVIL_TEMPLATE
            } else {
                AGENTS_MD_TEMPLATE
            },
            &Context { dirs },
        )
        .map_err(ExecutionError::RenderHandlebars)?
        .as_bytes(),
    )
    .map_err(ExecutionError::AgentsFileFailed)?;

    Ok(())
}

#[derive(Serialize)]
struct Context {
    dirs: Vec<(String, String)>,
}
