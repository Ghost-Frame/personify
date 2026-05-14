//! Entry point for the `personify-server` binary.
//!
//! Parses configuration from environment variables, initializes tracing, wires
//! up backends via mock stubs (concrete adapters wired in milestone 2), and
//! calls [`personify_server::run`] to serve until SIGTERM/SIGINT.

use std::sync::Arc;

use mimalloc::MiMalloc;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

use personify_server::{AppState, LogFormat, ServerConfig, ServerError};

/// Use mimalloc as the global allocator for improved throughput on
/// allocation-heavy workloads (many small async tasks).
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// Initialize the `tracing` subscriber based on the resolved [`ServerConfig`].
///
/// Applies an [`tracing_subscriber::EnvFilter`] from `config.log_level`.
/// Falls back to `info` if the level string is invalid. Emits either
/// structured JSON or compact text output depending on `config.log_format`.
fn init_tracing(config: &ServerConfig) {
    let env_filter = tracing_subscriber::EnvFilter::try_new(&config.log_level)
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    let registry = tracing_subscriber::registry().with(env_filter);

    match config.log_format {
        LogFormat::Json => registry
            .with(tracing_subscriber::fmt::layer().json())
            .init(),
        LogFormat::Text => registry.with(tracing_subscriber::fmt::layer()).init(),
    }
}

/// Build a placeholder [`AppState`] from the given config.
///
/// Concrete database and filesystem backends are wired in milestone 2.
/// This function provides no-op stubs so that the binary can start and serve
/// the skeleton endpoints (health, metrics, MCP placeholder) without any
/// external infrastructure.
async fn build_state(config: Arc<ServerConfig>) -> Result<AppState, ServerError> {
    // Placeholder backends -- replaced with real adapters in milestone 2.
    // The build_state function exists so that backend initialization errors
    // (connection refused, missing credentials) can be surfaced as
    // ServerError::Startup before the bind syscall.
    let catalog: Arc<dyn personify_catalog::CatalogBackend> = Arc::new(NoopCatalog);
    let objects: Arc<dyn personify_objects::PackStore> = Arc::new(NoopPackStore);

    Ok(AppState {
        catalog,
        objects,
        runtime: None,
        config,
    })
}

#[tokio::main]
async fn main() {
    let config = match ServerConfig::from_env() {
        Ok(c) => Arc::new(c),
        Err(e) => {
            eprintln!("configuration error: {e}");
            std::process::exit(2);
        }
    };
    // Note: `from_env` returns `Box<figment::Error>` to avoid large Err variants.

    init_tracing(&config);
    tracing::debug!(?config, "resolved server configuration");

    let state = match build_state(Arc::clone(&config)).await {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("startup failed: {e}");
            std::process::exit(3);
        }
    };

    if let Err(e) = personify_server::run(state).await {
        tracing::error!("server error: {e}");
        let code = match e {
            ServerError::Bind(_) => 2,
            ServerError::Startup(_) => 3,
            ServerError::Shutdown(_) => 1,
        };
        std::process::exit(code);
    }
}

// ---------------------------------------------------------------------------
// Placeholder backend stubs (replaced by real adapters in milestone 2)
// ---------------------------------------------------------------------------

/// No-op catalog backend used until the Postgres adapter is wired in.
///
/// Every method returns a `CatalogError::BackendError` indicating the backend
/// is not configured. This allows the binary to start and serve `/healthz`
/// with a `healthy: false` catalog status.
struct NoopCatalog;

#[async_trait::async_trait]
impl personify_catalog::CatalogBackend for NoopCatalog {
    async fn register_author(
        &self,
        _record: personify_catalog::AuthorRecord,
    ) -> Result<(), personify_catalog::CatalogError> {
        Err(personify_catalog::CatalogError::BackendError(
            "catalog not configured".into(),
        ))
    }

    async fn lookup_author(
        &self,
        _pubkey: &personify_catalog::Ed25519PublicKey,
    ) -> Result<personify_catalog::AuthorRecord, personify_catalog::CatalogError> {
        Err(personify_catalog::CatalogError::BackendError(
            "catalog not configured".into(),
        ))
    }

    async fn lookup_author_by_handle(
        &self,
        _handle: &str,
    ) -> Result<personify_catalog::AuthorRecord, personify_catalog::CatalogError> {
        Err(personify_catalog::CatalogError::BackendError(
            "catalog not configured".into(),
        ))
    }

    async fn list_authors(
        &self,
        _limit: u32,
        _offset: u32,
    ) -> Result<Vec<personify_catalog::AuthorRecord>, personify_catalog::CatalogError> {
        Ok(Vec::new())
    }

    async fn register_pack_version(
        &self,
        _record: personify_catalog::PackVersionRecord,
    ) -> Result<(), personify_catalog::CatalogError> {
        Err(personify_catalog::CatalogError::BackendError(
            "catalog not configured".into(),
        ))
    }

    async fn get_pack(
        &self,
        _name: &str,
    ) -> Result<personify_catalog::PackRecord, personify_catalog::CatalogError> {
        Err(personify_catalog::CatalogError::BackendError(
            "catalog not configured".into(),
        ))
    }

    async fn get_pack_version(
        &self,
        _name: &str,
        _version: &str,
    ) -> Result<personify_catalog::PackVersionRecord, personify_catalog::CatalogError> {
        Err(personify_catalog::CatalogError::BackendError(
            "catalog not configured".into(),
        ))
    }

    async fn list_pack_versions(
        &self,
        _name: &str,
    ) -> Result<Vec<personify_catalog::PackVersionRecord>, personify_catalog::CatalogError> {
        Ok(Vec::new())
    }

    async fn search_packs(
        &self,
        _filters: &personify_catalog::PackSearchFilters,
    ) -> Result<Vec<personify_catalog::PackSearchResult>, personify_catalog::CatalogError> {
        Ok(Vec::new())
    }

    async fn increment_download_counter(
        &self,
        _name: &str,
        _version: &str,
    ) -> Result<u64, personify_catalog::CatalogError> {
        Err(personify_catalog::CatalogError::BackendError(
            "catalog not configured".into(),
        ))
    }

    async fn tombstone_pack(
        &self,
        _name: &str,
        _version: &str,
        _record: personify_catalog::TombstoneRecord,
    ) -> Result<(), personify_catalog::CatalogError> {
        Err(personify_catalog::CatalogError::BackendError(
            "catalog not configured".into(),
        ))
    }

    async fn get_handle_pubkey(
        &self,
        _handle: &str,
    ) -> Result<personify_catalog::Ed25519PublicKey, personify_catalog::CatalogError> {
        Err(personify_catalog::CatalogError::BackendError(
            "catalog not configured".into(),
        ))
    }

    async fn set_handle_pubkey(
        &self,
        _handle: &str,
        _pubkey: personify_catalog::Ed25519PublicKey,
    ) -> Result<(), personify_catalog::CatalogError> {
        Err(personify_catalog::CatalogError::BackendError(
            "catalog not configured".into(),
        ))
    }

    async fn health(
        &self,
    ) -> Result<personify_catalog::HealthStatus, personify_catalog::CatalogError> {
        Ok(personify_catalog::HealthStatus {
            healthy: false,
            detail: "catalog backend not configured".to_string(),
        })
    }
}

/// No-op object store backend used until the filesystem adapter is wired in.
///
/// Every method returns an `ObjectStoreError::BackendError` indicating the
/// store is not configured. The `health` method returns `healthy: false`.
struct NoopPackStore;

#[async_trait::async_trait]
impl personify_objects::PackStore for NoopPackStore {
    async fn put(
        &self,
        _hash: &personify_objects::ObjectHash,
        _bytes: &[u8],
    ) -> Result<(), personify_objects::ObjectStoreError> {
        Err(personify_objects::ObjectStoreError::BackendError(
            "object store not configured".into(),
        ))
    }

    async fn get(
        &self,
        _hash: &personify_objects::ObjectHash,
    ) -> Result<Vec<u8>, personify_objects::ObjectStoreError> {
        Err(personify_objects::ObjectStoreError::BackendError(
            "object store not configured".into(),
        ))
    }

    async fn exists(
        &self,
        _hash: &personify_objects::ObjectHash,
    ) -> Result<bool, personify_objects::ObjectStoreError> {
        Ok(false)
    }

    async fn delete(
        &self,
        hash: &personify_objects::ObjectHash,
    ) -> Result<(), personify_objects::ObjectStoreError> {
        Err(personify_objects::ObjectStoreError::NotFound { hash: *hash })
    }

    async fn list_prefix(
        &self,
        _prefix: &[u8],
        _limit: usize,
    ) -> Result<Vec<personify_objects::ObjectHash>, personify_objects::ObjectStoreError> {
        Ok(Vec::new())
    }

    async fn health(
        &self,
    ) -> Result<personify_objects::ObjectStoreHealth, personify_objects::ObjectStoreError> {
        Ok(personify_objects::ObjectStoreHealth {
            healthy: false,
            total_objects: None,
            total_bytes: None,
            detail: "object store not configured".to_string(),
        })
    }
}
