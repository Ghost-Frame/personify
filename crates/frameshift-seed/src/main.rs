//! One-shot seeder for the frameshift catalog and object store.
//!
//! Reads persona directories from a configurable root path, builds a pack for
//! each directory that contains an `AGENTS.md` file plus a `pack.toml` manifest
//! (or synthesizes one), signs it with a generated Ed25519 key, stores the
//! canonical pack bytes in the object store, and registers the pack version and
//! author in the catalog.
//!
//! # Usage
//!
//! ```text
//! POSTGRES_URL=postgres://... \
//! OBJECT_STORE_ROOT=/tmp/frameshift-objects \
//! PERSONAS_ROOT=/path/to/personas \
//! frameshift-seed
//! ```
//!
//! All three environment variables are required. `OBJECT_STORE_ROOT` defaults
//! to `/tmp/frameshift-objects` when absent.
//!
//! # Key management
//!
//! On first run the seeder generates a fresh Ed25519 signing keypair and writes
//! the secret seed bytes to `$OBJECT_STORE_ROOT/../seed-signing-key.bin` (32
//! raw bytes). Subsequent runs that find this file load the same key, producing
//! stable author pubkey and signatures across re-seeds.
//!
//! # Idempotency
//!
//! The seeder is safe to run multiple times. `register_author` is idempotent for
//! an identical (pubkey, handle) pair. `register_pack_version` returns
//! `CatalogError::Conflict` when the (pack_name, version) pair already exists --
//! the seeder logs a warning and continues to the next persona.

use std::path::{Path, PathBuf};
use std::time::Duration;

use chrono::Utc;
use ed25519_dalek::{SigningKey, VerifyingKey};
use frameshift_catalog::{
    AuthorRecord, CatalogBackend, CatalogError, Ed25519PublicKey, PackStatus, PackVersionRecord,
};
use frameshift_catalog_postgres::{PostgresCatalog, PostgresCatalogConfig};
use frameshift_objects::PackStore;
use frameshift_objects_fs::{FsPackStore, FsPackStoreConfig};
use frameshift_pack::{ObjectHash, Pack};
use secrecy::SecretString;
use tracing::{error, info, warn};

/// Errors produced by the seeder.
#[derive(Debug, thiserror::Error)]
enum SeedError {
    #[error("environment variable {0} is required but not set")]
    MissingEnv(&'static str),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("catalog error: {0}")]
    Catalog(#[from] CatalogError),

    #[error("pack error: {0}")]
    Pack(#[from] frameshift_pack::PackError),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("object store error: {0}")]
    Objects(#[from] frameshift_objects::ObjectStoreError),
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    if let Err(e) = run().await {
        error!("seeder failed: {e}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), SeedError> {
    let postgres_url = std::env::var("POSTGRES_URL")
        .map_err(|_| SeedError::MissingEnv("POSTGRES_URL"))?;

    let object_store_root = std::env::var("OBJECT_STORE_ROOT")
        .unwrap_or_else(|_| "/tmp/frameshift-objects".to_string());

    let personas_root = std::env::var("PERSONAS_ROOT")
        .map_err(|_| SeedError::MissingEnv("PERSONAS_ROOT"))?;

    info!("connecting to postgres");
    let catalog = PostgresCatalog::new(PostgresCatalogConfig {
        url: SecretString::new(postgres_url.clone()),
        pool_size: 5,
        connect_timeout: Duration::from_secs(10),
        statement_timeout: Duration::from_secs(30),
    })
    .await?;

    info!("opening object store at {object_store_root}");
    let objects = FsPackStore::new(FsPackStoreConfig {
        root: PathBuf::from(&object_store_root),
        verify_on_read: true,
        max_bytes: None,
        fsync_on_put: false,
    })
    .await?;

    let key_path = PathBuf::from(&object_store_root)
        .parent()
        .unwrap_or(Path::new("/tmp"))
        .join("seed-signing-key.bin");

    let signing_key = load_or_create_signing_key(&key_path)?;
    let verifying_key = signing_key.verifying_key();
    let author_pubkey = Ed25519PublicKey(verifying_key.to_bytes());

    info!("author pubkey: {author_pubkey}");

    let author_handle = "seed-author";
    register_author(&catalog, author_pubkey, author_handle).await?;

    let personas_path = PathBuf::from(&personas_root);
    let mut seeded = 0usize;
    let mut skipped = 0usize;

    for entry in std::fs::read_dir(&personas_path)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let persona_toml = path.join("persona.toml");
        let agents_md = path.join("AGENTS.md");
        if !persona_toml.exists() && !agents_md.exists() {
            continue;
        }

        let dir_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        // Skip hidden/symlink dirs and non-slug names.
        if dir_name.starts_with('.') {
            continue;
        }

        // Synthesize a pack.toml if one does not exist.
        let pack_toml_path = path.join("pack.toml");
        if !pack_toml_path.exists() {
            write_synthetic_pack_toml(&pack_toml_path, dir_name, author_handle, &verifying_key)?;
        }

        match seed_persona(
            &path,
            &catalog,
            &objects,
            &signing_key,
            author_pubkey,
        )
        .await
        {
            Ok(()) => {
                info!("seeded persona: {dir_name}");
                seeded += 1;
            }
            Err(SeedError::Catalog(CatalogError::Conflict { .. })) => {
                warn!("persona {dir_name}: already registered, skipping");
                skipped += 1;
            }
            Err(e) => {
                error!("persona {dir_name}: failed -- {e}");
                skipped += 1;
            }
        }
    }

    info!("pack versions seeded: seeded={seeded} skipped={skipped}");

    // Post-seed: update pack descriptions and tags from persona.toml files.
    info!("updating pack descriptions from persona.toml files");
    update_pack_metadata(&postgres_url, &personas_path).await?;

    info!("done");
    Ok(())
}

/// Register the seed author, treating idempotent re-registration as success.
async fn register_author(
    catalog: &PostgresCatalog,
    pubkey: Ed25519PublicKey,
    handle: &str,
) -> Result<(), SeedError> {
    let record = AuthorRecord {
        pubkey,
        handle: handle.to_string(),
        display_name: Some("Seed Author".to_string()),
        created_at: Utc::now(),
        oauth_links: vec![],
    };

    match catalog.register_author(record).await {
        Ok(()) => {
            info!("registered author: {handle}");
            Ok(())
        }
        Err(CatalogError::Conflict { .. }) | Err(CatalogError::HandleTaken { .. }) => {
            info!("author {handle} already registered");
            Ok(())
        }
        Err(e) => Err(SeedError::Catalog(e)),
    }
}

/// Build and seed a single persona directory.
///
/// Steps:
/// 1. Load Pack from directory (requires pack.toml to exist).
/// 2. Sign with signing key.
/// 3. Compute canonical bytes for the object store.
/// 4. Store bytes via PackStore.
/// 5. Register pack version in catalog.
async fn seed_persona(
    dir: &Path,
    catalog: &PostgresCatalog,
    objects: &FsPackStore,
    signing_key: &SigningKey,
    author_pubkey: Ed25519PublicKey,
) -> Result<(), SeedError> {
    let mut pack = Pack::from_dir(dir)?;
    let signature = pack.sign(signing_key)?;

    let canonical_bytes = pack_canonical_bytes(dir)?;
    let content_hash = ObjectHash::of(&canonical_bytes);

    // Verify the content hash matches the pack's canonical hash.
    let pack_hash = ObjectHash::from_bytes(pack.canonical_hash());
    if content_hash != pack_hash {
        // This should never happen -- both are SHA-256 of the same data.
        return Err(SeedError::Io(std::io::Error::other(format!(
            "content hash mismatch: {content_hash} != {pack_hash}"
        ))));
    }

    objects.put(&content_hash, &canonical_bytes).await?;

    let manifest = pack.manifest();
    let cap_json = manifest
        .capability_manifest
        .as_ref()
        .map(serde_json::to_string)
        .transpose()?
        .unwrap_or_else(|| "{}".to_string());

    let version_record = PackVersionRecord {
        pack_name: manifest.name.clone(),
        version: manifest.version.clone(),
        content_hash,
        signature: signature.to_bytes().to_vec(),
        author_pubkey,
        parent_hash: None,
        capability_manifest_json: cap_json,
        schema_version: manifest.schema_version,
        license: manifest.license.clone().unwrap_or_else(|| "UNKNOWN".to_string()),
        published_at: Utc::now(),
        status: PackStatus::Active,
        size_bytes: canonical_bytes.len() as u64,
    };

    catalog.register_pack_version(version_record).await?;
    Ok(())
}

/// Serialize a pack directory into a canonical byte stream.
///
/// The byte stream is the same data that the canonical hash function hashes:
/// for each entry (sorted byte-lexicographically by normalized path, excluding
/// `signature.sig`): `path NUL length NUL bytes NUL`.
///
/// This is the byte content stored in the object store. The SHA-256 of this
/// byte stream equals the pack's canonical hash.
fn pack_canonical_bytes(dir: &Path) -> Result<Vec<u8>, SeedError> {
    // Re-implement the serialization by reading directory entries the same way
    // the canonical module does, then building the byte stream.
    use std::collections::BTreeMap;

    const SIGNATURE_FILENAME: &str = "signature.sig";
    const MAX_FILE_SIZE: u64 = 1024 * 1024;

    let mut entries: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    collect_entries_for_bytes(dir, dir, &mut entries, MAX_FILE_SIZE)?;

    let mut out = Vec::new();
    for (path, content) in &entries {
        if path == SIGNATURE_FILENAME {
            continue;
        }
        out.extend_from_slice(path.as_bytes());
        out.push(0);
        out.extend_from_slice(content.len().to_string().as_bytes());
        out.push(0);
        out.extend_from_slice(content);
        out.push(0);
    }

    Ok(out)
}

/// Recursively collect files into a BTreeMap (keyed by normalized path) for
/// canonical byte serialization.
fn collect_entries_for_bytes(
    base: &Path,
    current: &Path,
    entries: &mut std::collections::BTreeMap<String, Vec<u8>>,
    max_file_size: u64,
) -> Result<(), SeedError> {
    use unicode_normalization::UnicodeNormalization as _;

    for entry in std::fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let ft = entry.file_type()?;

        if ft.is_dir() {
            collect_entries_for_bytes(base, &path, entries, max_file_size)?;
            continue;
        }
        if !ft.is_file() {
            continue;
        }

        let rel = path
            .strip_prefix(base)
            .expect("path is under base")
            .to_str()
            .ok_or_else(|| {
                SeedError::Io(std::io::Error::other(format!(
                    "non-UTF-8 path: {}",
                    path.display()
                )))
            })?;

        let normalized: String = rel.nfc().collect();
        let canonical = normalized
            .replace('\\', "/")
            .strip_prefix("./")
            .map(|s| s.to_string())
            .unwrap_or(normalized.replace('\\', "/"));

        if canonical == "signature.sig" {
            continue;
        }

        let content = std::fs::read(&path)?;
        if content.len() as u64 > max_file_size {
            warn!(
                "file {} exceeds max size ({} bytes), skipping",
                canonical,
                content.len()
            );
            continue;
        }

        entries.insert(canonical, content);
    }

    Ok(())
}

/// Load a signing key from disk, or generate a new one and persist it.
fn load_or_create_signing_key(path: &Path) -> Result<SigningKey, SeedError> {
    if path.exists() {
        let bytes = std::fs::read(path)?;
        let seed: [u8; 32] = bytes.try_into().map_err(|_| {
            SeedError::Io(std::io::Error::other(
                "signing key file must be exactly 32 bytes",
            ))
        })?;
        info!("loaded signing key from {}", path.display());
        Ok(SigningKey::from_bytes(&seed))
    } else {
        let key = SigningKey::generate(&mut rand_core::OsRng);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, key.to_bytes())?;
        info!("generated new signing key at {}", path.display());
        Ok(key)
    }
}

/// Write a synthetic `pack.toml` manifest for a persona directory.
///
/// The manifest is minimal but valid. The `author_pubkey` field is encoded as
/// the verifying key's byte representation in hex (the pack manifest stores it
/// as a string -- it is informational only, not parsed by the catalog which
/// uses the typed `Ed25519PublicKey`).
fn write_synthetic_pack_toml(
    path: &Path,
    dir_name: &str,
    author_handle: &str,
    verifying_key: &VerifyingKey,
) -> Result<(), SeedError> {
    let pubkey_hex: String = verifying_key
        .to_bytes()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();

    let content = format!(
        r#"schema_version = 1
name = "{dir_name}"
author_handle = "{author_handle}"
author_pubkey = "{pubkey_hex}"
version = "0.1.0"
license = "Elastic-2.0"
"#
    );

    std::fs::write(path, content)?;
    info!("wrote synthetic pack.toml for {dir_name}");
    Ok(())
}

/// Minimal persona.toml structure for extracting description and stack categories.
#[derive(Debug, serde::Deserialize)]
struct PersonaToml {
    name: String,
    #[serde(default)]
    description: String,
}

/// Minimal patterns.toml structure for extracting stack categories as tags.
#[derive(Debug, serde::Deserialize)]
struct PatternsToml {
    #[serde(default)]
    stack: Vec<StackEntry>,
}

/// A single stack category entry.
#[derive(Debug, serde::Deserialize)]
struct StackEntry {
    category: String,
    #[serde(default)]
    items: Vec<toml::Value>,
}

/// Post-seed pass: read persona.toml from each directory, extract description
/// and derive tags from patterns.toml stack categories, then UPDATE the packs
/// table directly. Uses tokio-postgres for the raw UPDATE since the catalog
/// trait does not expose a pack metadata update method.
async fn update_pack_metadata(postgres_url: &str, personas_root: &Path) -> Result<(), SeedError> {
    let (client, connection) = tokio_postgres::connect(postgres_url, tokio_postgres::NoTls)
        .await
        .map_err(|e| SeedError::Io(std::io::Error::other(format!("pg connect: {e}"))))?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("pg connection error: {e}");
        }
    });

    for entry in std::fs::read_dir(personas_root)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let persona_path = path.join("persona.toml");
        if !persona_path.exists() {
            continue;
        }

        let dir_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let persona_content = std::fs::read_to_string(&persona_path)?;
        let persona: PersonaToml = toml::from_str(&persona_content).map_err(|e| {
            SeedError::Io(std::io::Error::other(format!(
                "parse persona.toml for {dir_name}: {e}"
            )))
        })?;

        // Derive tags from patterns.toml stack categories.
        let mut tags: Vec<String> = Vec::new();
        let patterns_path = path.join("patterns.toml");
        if patterns_path.exists() {
            let patterns_content = std::fs::read_to_string(&patterns_path)?;
            if let Ok(patterns) = toml::from_str::<PatternsToml>(&patterns_content) {
                for stack in &patterns.stack {
                    tags.push(stack.category.clone());
                }
            }
        }

        let description = if persona.description.is_empty() {
            format!("{} persona for AI coding agents", dir_name)
        } else {
            persona.description
        };

        let result = client
            .execute(
                "UPDATE packs SET description = $1, tags = $2 WHERE name = $3",
                &[&description, &tags, &persona.name],
            )
            .await;

        match result {
            Ok(rows) if rows > 0 => {
                info!("updated metadata for {dir_name}: {} tags", tags.len());
            }
            Ok(_) => {
                warn!("no pack row found for {dir_name}, skipping metadata update");
            }
            Err(e) => {
                warn!("failed to update metadata for {dir_name}: {e}");
            }
        }
    }

    Ok(())
}
