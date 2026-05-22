//! CLI handler for the `frameshift use <name>` subcommand.
//!
//! Activates the named persona and prints the rendered claude-target content
//! to stdout so callers can pipe or review it immediately.

use clap::Args;
use frameshift_client::Client;

use crate::util::CliError;

/// Arguments for the `use` subcommand.
#[derive(Debug, Args)]
pub struct UseArgs {
    /// Name of the persona to activate.
    pub name: String,
}

/// Execute the `use` subcommand.
///
/// Calls `client.activate` to write the active marker, then reads and prints
/// the rendered output for the `claude` target.
pub fn run_use(client: &Client, args: UseArgs) -> Result<(), CliError> {
    let project_root = std::env::current_dir()?;

    // Activate the persona (syncs the lock first, then writes the active marker).
    client.activate(&project_root, &args.name)?;

    // Read and print the rendered persona for the claude target.
    let rendered = client.rendered_persona(&project_root, &args.name, "claude")?;
    println!("{}", rendered);

    Ok(())
}
