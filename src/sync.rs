//! MongoDB sync module for the DataWallet PAO indexing system.
//!
//! Stores the full [`IndexingSystem`] as a single MongoDB document
//! (identified by `_id: "pao-indexing-system"`). Provides push/pull/sync
//! operations so that the local file-based state can be synced to and
//! from a remote MongoDB instance.
//!
//! # Usage
//!
//! ## Desktop (direct sync)
//! ```ignore
//! let sync = MongoSync::connect(MongoConfig::default())?;
//! let merged = sync.sync(&local_system)?;
//! ```
//!
//! ## Fullstack (server functions)
//! ```ignore
//! // From client-side Dioxus component:
//! let result = push_to_mongodb(json).await?;
//! let remote = pull_from_mongodb().await?;
//! ```

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// ── Public types ──────────────────────────────────────────────

/// Connection configuration for MongoDB.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MongoConfig {
    /// Connection URI, e.g. `mongodb://localhost:27017`.
    pub uri: String,
    /// Database name.
    pub database: String,
    /// Collection name inside the database.
    pub collection: String,
    /// Number of seconds before a connection attempt times out.
    pub connect_timeout_secs: u64,
}

impl Default for MongoConfig {
    fn default() -> Self {
        Self {
            uri: "mongodb://localhost:27017".into(),
            database: "datawallet".into(),
            collection: "pao_systems".into(),
            connect_timeout_secs: 5,
        }
    }
}

/// Sync result describing what happened during a push/pull.
#[derive(Clone, Debug, Default)]
pub struct SyncResult {
    /// Whether the operation succeeded.
    pub success: bool,
    /// A human-readable message.
    pub message: String,
    /// Number of entries that were transferred.
    pub entries_count: usize,
}

// ── Error type ────────────────────────────────────────────────

/// Errors that can occur during MongoDB operations.
#[derive(Debug)]
pub enum MongoError {
    Connection(String),
    Serialization(String),
    Deserialization(String),
    Read(String),
    Write(String),
    NotAvailable,
}

impl std::fmt::Display for MongoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MongoError::Connection(msg) => write!(f, "MongoDB connection error: {msg}"),
            MongoError::Serialization(msg) => write!(f, "MongoDB serialization error: {msg}"),
            MongoError::Deserialization(msg) => write!(f, "MongoDB deserialization error: {msg}"),
            MongoError::Read(msg) => write!(f, "MongoDB read error: {msg}"),
            MongoError::Write(msg) => write!(f, "MongoDB write error: {msg}"),
            MongoError::NotAvailable => write!(f, "MongoDB sync is not available (feature disabled)"),
        }
    }
}

impl std::error::Error for MongoError {}

// ── Server-Side Configuration Loading ─────────────────────────

#[cfg(feature = "server")]
pub fn load_mongo_config() -> MongoConfig {
    let config_path = "config/mongo.json";
    match std::fs::read_to_string(config_path) {
        Ok(json) => {
            serde_json::from_str(&json).unwrap_or_else(|e| {
                eprintln!("[MongoSync] Failed to parse {config_path}: {e} — using defaults");
                MongoConfig::default()
            })
        }
        Err(e) => {
            println!("[MongoSync] {config_path} not found ({e}) — using default config");
            MongoConfig::default()
        }
    }
}

// ── Main Sync Client ──────────────────────────────────────────

/// A client that synchronises the local [`IndexingSystem`] with a
/// MongoDB document.
#[derive(Clone, Debug)]
pub struct MongoSync {
    config: MongoConfig,

    #[cfg(feature = "server")]
    client: Option<mongodb::sync::Client>,
}

impl MongoSync {
    /// Create a new `MongoSync` by connecting to the given config.
    pub fn connect(config: MongoConfig) -> Result<Self, MongoError> {
        #[cfg(feature = "server")]
        {
            let client = mongodb::sync::Client::with_uri_str(&config.uri)
                .map_err(|e| MongoError::Connection(e.to_string()))?;

            // Ping to verify the connection is live.
            let db = client.database(&config.database);
            let ping_cmd = bson::doc! { "ping": 1 };

            let _ = db.run_command(ping_cmd)
                .run()
                .map_err(|e| MongoError::Connection(format!("Ping failed: {e}")))?;

            Ok(Self {
                config,
                client: Some(client),
            })
        }

        #[cfg(not(feature = "server"))]
        {
            let _ = config;
            Err(MongoError::NotAvailable)
        }
    }

    /// Create a disconnected stub (useful when MongoDB is not needed).
    pub fn disconnected(config: MongoConfig) -> Self {
        Self {
            config,
            #[cfg(feature = "server")]
            client: None,
        }
    }

    /// The configuration in use.
    pub fn config(&self) -> &MongoConfig {
        &self.config
    }

    // ── Push ──────────────────────────────────────────────────

    /// Push (upsert) the given indexing system to MongoDB as a single
    /// document with `_id = "pao-indexing-system"`.
    #[cfg(feature = "server")]
    pub fn push(&self, system: &crate::indexing::IndexingSystem) -> Result<SyncResult, MongoError> {
        let client = self.client.as_ref().ok_or_else(|| {
            MongoError::Connection("Not connected".to_string())
        })?;

        let collection = client
            .database(&self.config.database)
            .collection::<bson::Document>(&self.config.collection);

        let system_json = serde_json::to_value(system)
            .map_err(|e| MongoError::Serialization(e.to_string()))?;

        let mut doc = bson::to_document(&system_json)
            .map_err(|e| MongoError::Serialization(e.to_string()))?;

        doc.insert("_id", "pao-indexing-system");

        let filter = bson::doc! { "_id": "pao-indexing-system" };
        let options = mongodb::options::ReplaceOptions::builder()
            .upsert(true)
            .build();

        collection
            .replace_one(filter, doc)
            .with_options(options)
            .run()
            .map_err(|e| MongoError::Write(e.to_string()))?;

        Ok(SyncResult {
            success: true,
            message: format!("Pushed {} entries to MongoDB", system.len()),
            entries_count: system.len(),
        })
    }

    #[cfg(not(feature = "server"))]
    pub fn push(&self, _system: &crate::indexing::IndexingSystem) -> Result<SyncResult, MongoError> {
        Err(MongoError::NotAvailable)
    }

    // ── Pull ──────────────────────────────────────────────────

    /// Pull (load) the indexing system from the MongoDB document.
    /// Returns `Ok(None)` if no document exists yet.
    #[cfg(feature = "server")]
    pub fn pull(&self) -> Result<Option<crate::indexing::IndexingSystem>, MongoError> {
        let client = self.client.as_ref().ok_or_else(|| {
            MongoError::Connection("Not connected".to_string())
        })?;

        let collection = client
            .database(&self.config.database)
            .collection::<bson::Document>(&self.config.collection);

        let filter = bson::doc! { "_id": "pao-indexing-system" };
        let opt_doc = collection
            .find_one(filter)
            .run()
            .map_err(|e| MongoError::Read(e.to_string()))?;

        match opt_doc {
            None => Ok(None),
            Some(doc) => {
                let mut doc = doc;
                doc.remove("_id");

                let json_val = bson::from_document::<serde_json::Value>(doc)
                    .map_err(|e| MongoError::Deserialization(e.to_string()))?;

                let system: crate::indexing::IndexingSystem = serde_json::from_value(json_val)
                    .map_err(|e| MongoError::Deserialization(e.to_string()))?;

                Ok(Some(system))
            }
        }
    }

    #[cfg(not(feature = "server"))]
    pub fn pull(&self) -> Result<Option<crate::indexing::IndexingSystem>, MongoError> {
        Err(MongoError::NotAvailable)
    }

    // ── Sync ──────────────────────────────────────────────────

    /// Two-way sync: push local → MongoDB, and if no local data exists
    /// but remote data does, pull remote → local.
    #[cfg(feature = "server")]
    pub fn sync(
        &self,
        local: &crate::indexing::IndexingSystem,
    ) -> Result<crate::indexing::IndexingSystem, MongoError> {
        let remote = self.pull()?;

        match remote {
            None => {
                let result = self.push(local)?;
                println!("[MongoSync] Pushed local to MongoDB: {}", result.message);
                Ok(local.clone())
            }
            Some(remote_sys) => {
                if local.len() == 0 && remote_sys.len() > 0 {
                    println!(
                        "[MongoSync] Pulled {} entries from MongoDB (local was empty)",
                        remote_sys.len()
                    );
                    Ok(remote_sys)
                } else if remote_synced_after_local(local, &remote_sys) {
                    println!(
                        "[MongoSync] Remote is newer – pulled {} entries",
                        remote_sys.len()
                    );
                    Ok(remote_sys)
                } else {
                    let result = self.push(local)?;
                    println!("[MongoSync] Pushed local to MongoDB: {}", result.message);
                    Ok(local.clone())
                }
            }
        }
    }

    #[cfg(not(feature = "server"))]
    pub fn sync(
        &self,
        local: &crate::indexing::IndexingSystem,
    ) -> Result<crate::indexing::IndexingSystem, MongoError> {
        Err(MongoError::NotAvailable)
    }
}

// ── Dioxus Server Functions ───────────────────────────────────
//
// These async server functions allow client-side Dioxus components
// to call MongoDB operations over HTTP without directly connecting.

/// Server function to push (upsert) the local indexing system to MongoDB.
///
/// Called from client-side code as:
/// ```ignore
/// let result = push_to_mongodb(json_string).await?;
/// ```
#[server]
pub async fn push_to_mongodb(system_json: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use mongodb::{Client, options::ReplaceOptions};

        let config = load_mongo_config();

        let client = Client::with_uri_str(&config.uri).await
            .map_err(|e| ServerFnError::new(format!("Connection error: {}", e)))?;

        let db = client.database(&config.database);
        let collection = db.collection::<bson::Document>(&config.collection);

        let system_val: serde_json::Value = serde_json::from_str(&system_json)
            .map_err(|e| ServerFnError::new(format!("JSON parsing error: {}", e)))?;

        let mut doc = bson::to_document(&system_val)
            .map_err(|e| ServerFnError::new(format!("BSON conversion error: {}", e)))?;

        doc.insert("_id", "pao-indexing-system");

        let filter = bson::doc! { "_id": "pao-indexing-system" };
        let options = ReplaceOptions::builder()
            .upsert(true)
            .build();

        collection
            .replace_one(filter, doc)
            .with_options(options)
            .await
            .map_err(|e| ServerFnError::new(format!("MongoDB write error: {}", e)))?;

        Ok("Push successful".to_string())
    }
}

/// Server function to pull the indexing system from MongoDB.
///
/// Called from client-side code as:
/// ```ignore
/// if let Some(remote_json) = pull_from_mongodb().await? {
///     let system: IndexingSystem = serde_json::from_str(&remote_json)?;
/// }
/// ```
#[server]
pub async fn pull_from_mongodb() -> Result<Option<String>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use mongodb::Client;

        let config = load_mongo_config();

        let client = Client::with_uri_str(&config.uri).await
            .map_err(|e| ServerFnError::new(format!("Connection error: {}", e)))?;

        let db = client.database(&config.database);
        let collection = db.collection::<bson::Document>(&config.collection);

        let filter = bson::doc! { "_id": "pao-indexing-system" };
        let opt_doc = collection
            .find_one(filter)
            .await
            .map_err(|e| ServerFnError::new(format!("MongoDB read error: {}", e)))?;

        match opt_doc {
            None => Ok(None),
            Some(mut doc) => {
                doc.remove("_id");

                let json_val = bson::from_document::<serde_json::Value>(doc)
                    .map_err(|e| ServerFnError::new(format!("Deserialization error: {}", e)))?;

                let system_str = serde_json::to_string(&json_val)
                    .map_err(|e| ServerFnError::new(format!("JSON conversion error: {}", e)))?;

                Ok(Some(system_str))
            }
        }
    }
}

// ── Heuristic ─────────────────────────────────────────────────

/// Compare the history timestamps of two systems to guess which one
/// was modified more recently.
pub fn remote_synced_after_local(
    local: &crate::indexing::IndexingSystem,
    remote: &crate::indexing::IndexingSystem,
) -> bool {
    let local_latest = local
        .history
        .last()
        .map(|h| &h.timestamp[..]);
    let remote_latest = remote
        .history
        .last()
        .map(|h| &h.timestamp[..]);

    match (local_latest, remote_latest) {
        (Some(lt), Some(rt)) => rt > lt,
        (None, Some(_)) => true,
        _ => false,
    }
}