//! `frameshift migrate` subcommand.
//!
//! Triggers the legacy-file migration shim that moves pre-WS-1 project files
//! (`frameshift.toml`, `frameshift.lock`) from the project root into the
//! central store. The migration is normally triggered as a side-effect of
//! `client.project_paths()`; this command makes it explicit and prints a
//! human-readable summary of what was moved.

use clap::Args;

use frameshift_client::Client;

use crate::util::CliError;

/// Arguments for the `migrate` subcommand.
///
/// This subcommand takes no arguments; it operates on the current working
/// directory as the project root.
#[derive(Debug, Args)]
pub struct MigrateArgs {}

/// Execute the `migrate` subcommand.
///
/// Calls `client.project_paths(cwd)` which internally invokes
/// `migrate_legacy_project_files`. The migration shim copies any legacy
/// `frameshift.toml` / `frameshift.lock` from the project root into the
/// central store (if the central equivalents do not yet exist) and removes
/// the originals.
///
/// Because the migration side-effect is wired inside `project_paths`,
/// we do not need direct access to the private `migrate_legacy_project_files`
/// function -- calling `project_paths` is sufficient.
pub fn run_migrate(client: &Client, _args: MigrateArgs) -> Result<(), CliError> {
    let cwd = std::env::current_dir().map_err(|source| frameshift_client::ClientError::Io {
        path: std::path::PathBuf::from("."),
        source,
    })?;

    // project_paths triggers migrate_legacy_project_files internally.
    // We capture the paths for reporting purposes.
    let paths = client.project_paths(&cwd)?;

    println!("migrate: project id {}", paths.project_id);
    println!("  central store: {}", paths.project_state_dir.display());
    println!("  legacy files checked and migrated if present");
    Ok(())
}
