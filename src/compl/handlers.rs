//! Module related to completion handling.
//!
//! This module gathers all completion commands.

use anyhow::Result;
use clap::Command;
use clap_complete::Shell;
use std::io::stdout;

/// Generates completion script from the given command and shell.
pub fn generate(mut cmd: Command, shell: Shell) -> Result<()> {
    let name = cmd.get_name().to_string();
    clap_complete::generate(shell, &mut cmd, name, &mut stdout());
    Ok(())
}
