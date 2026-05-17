use frameshift_client::{Client, InstallRequest, InstallSource, PersonaSpec};
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let Some(command) = args.next() else {
        return Err(usage().to_string());
    };

    let client = Client::with_default_data_root().map_err(|error| error.to_string())?;

    match command.as_str() {
        "install" => {
            let spec = args
                .next()
                .ok_or_else(|| "missing required argument: <name@version>".to_string())?
                .parse::<PersonaSpec>()
                .map_err(|error| error.to_string())?;
            let mut source = InstallSource::Registry;

            while let Some(flag) = args.next() {
                if flag == "--from-path" {
                    let path = args
                        .next()
                        .ok_or_else(|| "missing value for --from-path".to_string())?;
                    source = InstallSource::LocalPath(PathBuf::from(path));
                    continue;
                }

                return Err(format!("unknown install argument: {flag}"));
            }

            let report = client
                .install(InstallRequest {
                    project_root: current_dir()?,
                    spec,
                    source,
                })
                .map_err(|error| error.to_string())?;
            println!(
                "installed {}@{} ({})",
                report.persona.name, report.persona.version, report.persona.hash
            );
            Ok(())
        }
        "activate" => {
            let persona = args
                .next()
                .ok_or_else(|| "missing required argument: <persona-name>".to_string())?;
            ensure_no_extra_args(args)?;
            client
                .activate(&current_dir()?, &persona)
                .map_err(|error| error.to_string())?;
            println!("activated {persona}");
            Ok(())
        }
        "sync" => {
            ensure_no_extra_args(args)?;
            let report = client
                .sync(&current_dir()?)
                .map_err(|error| error.to_string())?;
            println!("synced {} persona(s)", report.personas.len());
            Ok(())
        }
        "gc" => {
            ensure_no_extra_args(args)?;
            let report = client.gc().map_err(|error| error.to_string())?;
            println!("removed {} cache entries", report.removed_hashes.len());
            Ok(())
        }
        "project-id" => {
            ensure_no_extra_args(args)?;
            println!(
                "{}",
                client
                    .project_id(&current_dir()?)
                    .map_err(|error| error.to_string())?
            );
            Ok(())
        }
        _ => Err(usage().to_string()),
    }
}

fn current_dir() -> Result<PathBuf, String> {
    std::env::current_dir().map_err(|error| error.to_string())
}

fn ensure_no_extra_args(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    if let Some(extra) = args.next() {
        return Err(format!("unexpected argument: {extra}"));
    }
    Ok(())
}

fn usage() -> &'static str {
    "usage: frameshift <install|activate|sync|gc|project-id>\n       frameshift install <name@version> [--from-path <local-pack-dir>]"
}
