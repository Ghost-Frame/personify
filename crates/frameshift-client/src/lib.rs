mod error;
mod model;

pub use error::ClientError;
pub use model::{
    ClientOptions, GcReport, InstallReport, InstallRequest, InstallSource, LockedPersona, Lockfile,
    PersonaSpec, ProjectConfig, ProjectPaths, SyncReport, SCHEMA_VERSION,
};

use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::VerifyingKey;
use frameshift_pack::Pack;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Legacy filename written to the project root by pre-WS-1 versions.
/// Detected by the migration shim and moved into the central store.
const LEGACY_CONFIG_FILENAME: &str = "frameshift.toml";
/// Legacy filename written to the project root by pre-WS-1 versions.
const LEGACY_LOCK_FILENAME: &str = "frameshift.lock";
/// Canonical config filename inside the central store: `projects/<id>/config.toml`.
const CENTRAL_CONFIG_FILENAME: &str = "config.toml";
/// Canonical lock filename inside the central store: `projects/<id>/lock.toml`.
const CENTRAL_LOCK_FILENAME: &str = "lock.toml";
const ACTIVE_FILENAME: &str = "active";
/// Env var to override the auto-derived path-hash project_id.
const PROJECT_ID_ENV: &str = "FRAMESHIFT_PROJECT_ID";

const RENDER_TARGETS: [(&str, &str); 4] = [
    ("claude", "CLAUDE.md"),
    ("codex", "AGENTS.md"),
    ("gemini", "GEMINI.md"),
    ("generic", "AGENTS.md"),
];

const RENDER_CANDIDATES: [&str; 4] = ["AGENTS.md", "CLAUDE.md", "GEMINI.md", "README.md"];

pub struct Client {
    data_root: PathBuf,
}

impl Client {
    pub fn new(options: ClientOptions) -> Self {
        Self {
            data_root: options.data_root,
        }
    }

    pub fn with_default_data_root() -> Result<Self, ClientError> {
        Ok(Self::new(ClientOptions {
            data_root: default_data_root()?,
        }))
    }

    pub fn data_root(&self) -> &Path {
        &self.data_root
    }

    pub fn project_id(&self, project_root: &Path) -> Result<String, ClientError> {
        if let Ok(explicit) = std::env::var(PROJECT_ID_ENV) {
            if !explicit.is_empty() {
                validate_explicit_project_id(&explicit)?;
                return Ok(explicit);
            }
        }

        hashed_project_id(project_root)
    }

    pub fn project_paths(&self, project_root: &Path) -> Result<ProjectPaths, ClientError> {
        let project_id = self.project_id(project_root)?;
        let cache_dir = self.data_root.join("cache");
        let project_state_dir = self.data_root.join("projects").join(&project_id);
        let personas_dir = project_state_dir.join("personas");

        let paths = ProjectPaths {
            project_root: project_root.to_path_buf(),
            project_id,
            config_path: project_state_dir.join(CENTRAL_CONFIG_FILENAME),
            lock_path: project_state_dir.join(CENTRAL_LOCK_FILENAME),
            cache_dir,
            active_path: project_state_dir.join(ACTIVE_FILENAME),
            personas_dir,
            project_state_dir,
        };

        migrate_legacy_project_files(project_root, &paths);
        Ok(paths)
    }

    pub fn install(&self, request: InstallRequest) -> Result<InstallReport, ClientError> {
        ensure_exists(&request.project_root)?;

        let paths = self.project_paths(&request.project_root)?;
        let locked = match &request.source {
            InstallSource::LocalPath(pack_dir) => {
                let pack = Pack::from_dir(pack_dir)?;
                validate_pack_request(&pack, &request.spec)?;
                verify_pack_signature_if_present(&pack)?;
                let hash = pack.canonical_hash_hex();
                let cache_path = paths.cache_dir.join(&hash);
                ensure_cached_pack(pack_dir, &cache_path)?;
                locked_persona_from_pack(&pack)
            }
            InstallSource::Registry => return Err(ClientError::RegistryInstallNotImplemented),
        };

        let mut lockfile = load_lockfile(&paths.lock_path)?.unwrap_or_default();
        upsert_locked_persona(&mut lockfile, locked.clone());
        let raw_lock = toml::to_string_pretty(&lockfile)?;
        self.materialize_project_state(&paths, &lockfile, &raw_lock)?;

        Ok(InstallReport {
            project_id: paths.project_id,
            cache_path: paths.cache_dir.join(&locked.hash),
            persona: locked,
        })
    }

    pub fn activate(&self, project_root: &Path, persona: &str) -> Result<(), ClientError> {
        let report = self.sync(project_root)?;
        if !report.personas.iter().any(|installed| installed == persona) {
            return Err(ClientError::PersonaNotInstalled(persona.to_string()));
        }

        let paths = self.project_paths(project_root)?;
        ensure_dir(&paths.project_state_dir)?;
        write_file(&paths.active_path, persona.as_bytes())
    }

    pub fn sync(&self, project_root: &Path) -> Result<SyncReport, ClientError> {
        let paths = self.project_paths(project_root)?;
        let Some((raw_lock, lockfile)) = load_lockfile_with_raw(&paths.lock_path)? else {
            return Ok(SyncReport {
                project_id: paths.project_id,
                personas: Vec::new(),
            });
        };

        self.materialize_project_state(&paths, &lockfile, &raw_lock)?;
        Ok(SyncReport {
            project_id: paths.project_id,
            personas: lockfile
                .personas
                .iter()
                .map(|persona| persona.name.clone())
                .collect(),
        })
    }

    pub fn gc(&self) -> Result<GcReport, ClientError> {
        let mut referenced_hashes = BTreeSet::new();
        let projects_root = self.data_root.join("projects");

        if projects_root.exists() {
            for entry in read_dir_sorted(&projects_root)? {
                let project_dir = entry.path();
                if !entry
                    .file_type()
                    .map_err(|source| ClientError::Io {
                        path: project_dir.clone(),
                        source,
                    })?
                    .is_dir()
                {
                    continue;
                }

                let central_lock = project_dir.join(CENTRAL_LOCK_FILENAME);
                if let Some(lockfile) = load_lockfile(&central_lock)? {
                    for persona in lockfile.personas {
                        referenced_hashes.insert(persona.hash);
                    }
                }
            }
        }

        let mut removed_hashes = Vec::new();
        let cache_root = self.data_root.join("cache");
        if cache_root.exists() {
            for entry in read_dir_sorted(&cache_root)? {
                let path = entry.path();
                if !entry
                    .file_type()
                    .map_err(|source| ClientError::Io {
                        path: path.clone(),
                        source,
                    })?
                    .is_dir()
                {
                    continue;
                }

                let hash = entry.file_name().to_string_lossy().to_string();
                if !referenced_hashes.contains(&hash) {
                    debug!(hash, "removing unreferenced cache entry");
                    remove_dir_all(&path)?;
                    removed_hashes.push(hash);
                }
            }
        }

        Ok(GcReport { removed_hashes })
    }

    fn materialize_project_state(
        &self,
        paths: &ProjectPaths,
        lockfile: &Lockfile,
        raw_lock: &str,
    ) -> Result<(), ClientError> {
        ensure_dir(&paths.cache_dir)?;
        ensure_dir(&paths.personas_dir)?;
        // Lock file lives only in the central store -- nothing is written to the project root.
        write_file(&paths.lock_path, raw_lock.as_bytes())?;

        let expected_names: BTreeSet<&str> =
            lockfile.personas.iter().map(|p| p.name.as_str()).collect();
        for entry in read_dir_sorted(&paths.personas_dir)? {
            let path = entry.path();
            if !entry
                .file_type()
                .map_err(|source| ClientError::Io {
                    path: path.clone(),
                    source,
                })?
                .is_dir()
            {
                continue;
            }

            let name = entry.file_name().to_string_lossy().to_string();
            if !expected_names.contains(name.as_str()) {
                remove_dir_all(&path)?;
            }
        }

        for persona in &lockfile.personas {
            let cache_path = paths.cache_dir.join(&persona.hash);
            if !cache_path.exists() {
                return Err(ClientError::MissingCacheEntry {
                    hash: persona.hash.clone(),
                    path: cache_path,
                });
            }

            let persona_dir = paths.personas_dir.join(&persona.name);
            if persona_dir.exists() {
                remove_dir_all(&persona_dir)?;
            }
            ensure_dir(&persona_dir)?;

            let source_dir = persona_dir.join("source");
            copy_dir_recursive(&cache_path, &source_dir)?;

            let rendered_root = persona_dir.join("rendered");
            materialize_rendered_outputs(&cache_path, &rendered_root)?;

            // Growth is local-only and append-only -- a single file per persona, never published upstream.
            touch_empty(&persona_dir.join("growth.md"))?;
        }

        if paths.active_path.exists() {
            let active_name = read_to_string(&paths.active_path)?.trim().to_string();
            if !active_name.is_empty()
                && !lockfile
                    .personas
                    .iter()
                    .any(|persona| persona.name == active_name)
            {
                remove_file_if_exists(&paths.active_path)?;
            }
        }

        Ok(())
    }
}

fn default_data_root() -> Result<PathBuf, ClientError> {
    if let Ok(xdg_data_home) = std::env::var("XDG_DATA_HOME") {
        if !xdg_data_home.is_empty() {
            return Ok(PathBuf::from(xdg_data_home).join("frameshift"));
        }
    }

    let home = std::env::var("HOME")
        .map(PathBuf::from)
        .map_err(|source| ClientError::Io {
            path: PathBuf::from("$HOME"),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, source),
        })?;
    Ok(home.join(".local").join("share").join("frameshift"))
}

fn validate_explicit_project_id(project_id: &str) -> Result<(), ClientError> {
    if project_id.is_empty() || project_id == "." || project_id == ".." || project_id.contains('/')
    {
        return Err(ClientError::InvalidProjectId(project_id.to_string()));
    }

    if project_id.contains('\\') {
        return Err(ClientError::InvalidProjectId(project_id.to_string()));
    }

    Ok(())
}

/// Best-effort migration: if a pre-WS-1 install left `frameshift.toml` or
/// `frameshift.lock` at the project root, copy each into the central store
/// (if the central equivalent does not yet exist) and remove the original.
///
/// Failures are logged via `tracing::warn!` and swallowed -- migration must
/// never panic or block the calling operation.
fn migrate_legacy_project_files(project_root: &Path, paths: &ProjectPaths) {
    let mut migrated_any = false;
    let pairs: [(&str, &Path); 2] = [
        (LEGACY_CONFIG_FILENAME, &paths.config_path),
        (LEGACY_LOCK_FILENAME, &paths.lock_path),
    ];

    for (legacy_name, central_path) in pairs {
        let legacy_path = project_root.join(legacy_name);
        if !legacy_path.exists() {
            continue;
        }

        if !central_path.exists() {
            if let Some(parent) = central_path.parent() {
                if let Err(error) = fs::create_dir_all(parent) {
                    warn!(
                        path = %parent.display(),
                        error = %error,
                        "failed to create central-store directory during legacy migration"
                    );
                    continue;
                }
            }

            match fs::read(&legacy_path) {
                Ok(bytes) => {
                    if let Err(error) = fs::write(central_path, &bytes) {
                        warn!(
                            path = %central_path.display(),
                            error = %error,
                            "failed to write central-store copy during legacy migration"
                        );
                        continue;
                    }
                }
                Err(error) => {
                    warn!(
                        path = %legacy_path.display(),
                        error = %error,
                        "failed to read legacy project-root file during migration"
                    );
                    continue;
                }
            }
        }

        match fs::remove_file(&legacy_path) {
            Ok(()) => {
                migrated_any = true;
            }
            Err(error) => {
                warn!(
                    path = %legacy_path.display(),
                    error = %error,
                    "failed to remove legacy project-root file after migration"
                );
            }
        }
    }

    if migrated_any {
        info!(
            project_root = %project_root.display(),
            "migrated legacy frameshift.toml/lock from project root to central store"
        );
    }
}

fn hashed_project_id(project_root: &Path) -> Result<String, ClientError> {
    let canonical_root = fs::canonicalize(project_root).map_err(|source| ClientError::Io {
        path: project_root.to_path_buf(),
        source,
    })?;
    let canonical_str = canonical_root
        .to_str()
        .ok_or_else(|| ClientError::NonUtf8Path(canonical_root.clone()))?;
    let digest = Sha256::digest(canonical_str.as_bytes());
    Ok(hex::encode(digest))
}

fn validate_pack_request(pack: &Pack, spec: &PersonaSpec) -> Result<(), ClientError> {
    let manifest = pack.manifest();
    if manifest.name != spec.name || manifest.version != spec.version {
        return Err(ClientError::ManifestMismatch {
            expected_name: spec.name.clone(),
            expected_version: spec.version.clone(),
            actual_name: manifest.name.clone(),
            actual_version: manifest.version.clone(),
        });
    }
    Ok(())
}

fn verify_pack_signature_if_present(pack: &Pack) -> Result<(), ClientError> {
    if !pack.has_signature() {
        return Ok(());
    }

    let key_bytes = parse_verifying_key_bytes(&pack.manifest().author_pubkey)?;
    let key = VerifyingKey::from_bytes(&key_bytes)
        .map_err(|_| ClientError::InvalidAuthorPublicKey(pack.manifest().author_pubkey.clone()))?;
    pack.verify(&key)
        .map_err(|_| ClientError::SignatureVerification)
}

fn parse_verifying_key_bytes(encoded: &str) -> Result<[u8; 32], ClientError> {
    if let Ok(bytes) = hex::decode(encoded) {
        if let Ok(array) = <[u8; 32]>::try_from(bytes.as_slice()) {
            return Ok(array);
        }
    }

    if let Ok(bytes) = general_purpose::URL_SAFE_NO_PAD.decode(encoded) {
        if let Ok(array) = <[u8; 32]>::try_from(bytes.as_slice()) {
            return Ok(array);
        }
    }

    if let Ok(bytes) = general_purpose::STANDARD_NO_PAD.decode(encoded) {
        if let Ok(array) = <[u8; 32]>::try_from(bytes.as_slice()) {
            return Ok(array);
        }
    }

    Err(ClientError::InvalidAuthorPublicKey(encoded.to_string()))
}

fn locked_persona_from_pack(pack: &Pack) -> LockedPersona {
    let manifest = pack.manifest();
    LockedPersona {
        name: manifest.name.clone(),
        version: manifest.version.clone(),
        author_handle: manifest.author_handle.clone(),
        author_pubkey: manifest.author_pubkey.clone(),
        hash: pack.canonical_hash_hex(),
    }
}

fn upsert_locked_persona(lockfile: &mut Lockfile, persona: LockedPersona) {
    if let Some(existing) = lockfile
        .personas
        .iter_mut()
        .find(|existing| existing.name == persona.name)
    {
        *existing = persona;
        return;
    }

    lockfile.personas.push(persona);
    lockfile
        .personas
        .sort_by(|left, right| left.name.cmp(&right.name));
}

fn load_lockfile(path: &Path) -> Result<Option<Lockfile>, ClientError> {
    load_lockfile_with_raw(path).map(|maybe| maybe.map(|(_, lockfile)| lockfile))
}

fn load_lockfile_with_raw(path: &Path) -> Result<Option<(String, Lockfile)>, ClientError> {
    if !path.exists() {
        return Ok(None);
    }

    let raw = read_to_string(path)?;
    let lockfile = toml::from_str(&raw).map_err(|source| ClientError::TomlDeserialize {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(Some((raw, lockfile)))
}

fn ensure_cached_pack(source_dir: &Path, cache_path: &Path) -> Result<(), ClientError> {
    ensure_dir(
        cache_path
            .parent()
            .expect("cache paths are always nested under cache root"),
    )?;
    if cache_path.exists() {
        return Ok(());
    }

    let staging_path = cache_path.with_extension("tmp");
    if staging_path.exists() {
        remove_dir_all(&staging_path)?;
    }
    copy_dir_recursive(source_dir, &staging_path)?;

    match fs::rename(&staging_path, cache_path) {
        Ok(()) => Ok(()),
        Err(_source) if cache_path.exists() => {
            remove_dir_all(&staging_path)?;
            Ok(())
        }
        Err(source) => Err(ClientError::Io {
            path: cache_path.to_path_buf(),
            source,
        }),
    }
}

fn materialize_rendered_outputs(
    cache_path: &Path,
    rendered_root: &Path,
) -> Result<(), ClientError> {
    let render_source = find_render_source(cache_path)?;
    let content = fs::read(&render_source).map_err(|source| ClientError::Io {
        path: render_source.clone(),
        source,
    })?;

    for (target_dir, filename) in RENDER_TARGETS {
        let dir = rendered_root.join(target_dir);
        ensure_dir(&dir)?;
        write_file(&dir.join(filename), &content)?;
    }

    Ok(())
}

fn find_render_source(pack_dir: &Path) -> Result<PathBuf, ClientError> {
    for candidate in RENDER_CANDIDATES {
        let path = pack_dir.join(candidate);
        if path.is_file() {
            return Ok(path);
        }
    }

    for entry in read_dir_sorted(pack_dir)? {
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
            return Ok(path);
        }
    }

    Err(ClientError::MissingRenderSource(pack_dir.to_path_buf()))
}

fn ensure_exists(path: &Path) -> Result<(), ClientError> {
    if path.exists() {
        return Ok(());
    }

    Err(ClientError::Io {
        path: path.to_path_buf(),
        source: std::io::Error::new(std::io::ErrorKind::NotFound, "path does not exist"),
    })
}

fn ensure_dir(path: &Path) -> Result<(), ClientError> {
    fs::create_dir_all(path).map_err(|source| ClientError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn touch_empty(path: &Path) -> Result<(), ClientError> {
    if path.exists() {
        return Ok(());
    }
    write_file(path, b"")
}

fn write_file(path: &Path, bytes: &[u8]) -> Result<(), ClientError> {
    if let Some(parent) = path.parent() {
        ensure_dir(parent)?;
    }
    fs::write(path, bytes).map_err(|source| ClientError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn read_to_string(path: &Path) -> Result<String, ClientError> {
    fs::read_to_string(path).map_err(|source| ClientError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn remove_dir_all(path: &Path) -> Result<(), ClientError> {
    fs::remove_dir_all(path).map_err(|source| ClientError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn remove_file_if_exists(path: &Path) -> Result<(), ClientError> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(ClientError::Io {
            path: path.to_path_buf(),
            source,
        }),
    }
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<(), ClientError> {
    ensure_dir(destination)?;
    for entry in read_dir_sorted(source)? {
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry.file_type().map_err(|source| ClientError::Io {
            path: source_path.clone(),
            source,
        })?;

        if file_type.is_dir() {
            copy_dir_recursive(&source_path, &destination_path)?;
        } else if file_type.is_file() {
            let bytes = fs::read(&source_path).map_err(|source| ClientError::Io {
                path: source_path.clone(),
                source,
            })?;
            write_file(&destination_path, &bytes)?;
        }
    }
    Ok(())
}

fn read_dir_sorted(path: &Path) -> Result<Vec<fs::DirEntry>, ClientError> {
    let mut entries = fs::read_dir(path)
        .map_err(|source| ClientError::Io {
            path: path.to_path_buf(),
            source,
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|source| ClientError::Io {
            path: path.to_path_buf(),
            source,
        })?;
    entries.sort_by_key(|entry| entry.file_name());
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_persona_specs() {
        assert!("cryptographic".parse::<PersonaSpec>().is_err());
        assert!("@0.3.1".parse::<PersonaSpec>().is_err());
        assert!("cryptographic@".parse::<PersonaSpec>().is_err());
    }

    #[test]
    fn explicit_project_id_rejects_path_separators() {
        assert!(validate_explicit_project_id("team/alpha").is_err());
        assert!(validate_explicit_project_id("team\\alpha").is_err());
        assert!(validate_explicit_project_id("valid-id").is_ok());
    }
}
