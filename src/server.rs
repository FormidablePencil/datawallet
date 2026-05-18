//! Server functions for fullstack DataWallet
//!
//! This module contains all server-side operations that need to run on the server
//! (file I/O, MongoDB sync, etc.) and exposes them as server functions that can
//! be called from the client.

use dioxus::prelude::*;
use crate::indexing::IndexingSystem;

// ── Server Functions ──────────────────────────────────────────

/// Load the indexing system from the server's file storage
#[server]
pub async fn load_indexing_system() -> Result<IndexingSystem, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let sys = IndexingSystem::load_or_default();
        Ok(sys)
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(ServerFnError::new("Server operation not available in WASM"))
    }
}

/// Save the indexing system to the server's file storage AND sync to MongoDB
#[server]
pub async fn save_indexing_system(sys: IndexingSystem) -> Result<(), ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // 1. Save to local file
        sys.save().map_err(|e| ServerFnError::new(&format!("IO error: {}", e)))?;

        // 2. Push to MongoDB (best-effort, don't fail if MongoDB is unavailable)
        #[cfg(feature = "server")]
        {
            use mongodb::{Client, options::ReplaceOptions};
            use crate::sync::load_mongo_config;

            let config = load_mongo_config();
            if let Ok(client) = Client::with_uri_str(&config.uri).await {
                let db = client.database(&config.database);
                let collection = db.collection::<bson::Document>(&config.collection);

                if let Ok(system_val) = serde_json::to_value(&sys) {
                    if let Ok(mut doc) = bson::to_document(&system_val) {
                        doc.insert("_id", "pao-indexing-system");
                        let filter = bson::doc! { "_id": "pao-indexing-system" };
                        let options = ReplaceOptions::builder().upsert(true).build();
                        let _ = collection
                            .replace_one(filter, doc)
                            .with_options(options)
                            .await;
                        println!("[MongoSync] Pushed to MongoDB after save");
                    }
                }
            }
        }

        Ok(())
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(ServerFnError::new("Server operation not available in WASM"))
    }
}

/// Insert a new PAO entry
#[server]
pub async fn insert_entry(index: String, person: String, action: String, object: String) -> Result<(), ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut sys = IndexingSystem::load_or_default();
        sys.insert(&index, &person, &action, &object);
        save_indexing_system(sys).await
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(ServerFnError::new("Server operation not available in WASM"))
    }
}

/// Remove an entry
#[server]
pub async fn remove_entry(index: String) -> Result<(), ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut sys = IndexingSystem::load_or_default();
        sys.remove(&index);
        save_indexing_system(sys).await
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(ServerFnError::new("Server operation not available in WASM"))
    }
}

/// Reserve an index
#[server]
pub async fn reserve_index(index: String) -> Result<bool, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut sys = IndexingSystem::load_or_default();
        let reserved = sys.reserve(&index);
        save_indexing_system(sys).await?;
        Ok(reserved)
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(ServerFnError::new("Server operation not available in WASM"))
    }
}

/// Unreserve an index
#[server]
pub async fn unreserve_index(index: String) -> Result<(), ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut sys = IndexingSystem::load_or_default();
        sys.unreserve(&index);
        save_indexing_system(sys).await
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(ServerFnError::new("Server operation not available in WASM"))
    }
}

/// Add a branch route
#[server]
pub async fn add_branch_route(from: String, to: String) -> Result<(), ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut sys = IndexingSystem::load_or_default();
        sys.add_route(&format!("{}:branch", from), &to);
        save_indexing_system(sys).await
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(ServerFnError::new("Server operation not available in WASM"))
    }
}

/// Export the system to JSON
#[server]
pub async fn export_to_json() -> Result<String, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let sys = IndexingSystem::load_or_default();
        sys.export_to_json().map_err(|e| ServerFnError::new(&format!("Serialization error: {}", e)))
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(ServerFnError::new("Server operation not available in WASM"))
    }
}

/// Import the system from JSON
#[server]
pub async fn import_from_json(json: String) -> Result<(), ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let sys = IndexingSystem::import_from_json(&json)
            .map_err(|e| ServerFnError::new(&format!("Deserialization error: {}", e)))?;
        save_indexing_system(sys).await
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(ServerFnError::new("Server operation not available in WASM"))
    }
}

/// Capture a history entry
#[server]
pub async fn capture_history(
    index: String,
    entry_type: String,
    content: String,
    context: Option<String>,
) -> Result<(), ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut sys = IndexingSystem::load_or_default();
        sys.capture_history(&index, &entry_type, &content, context);
        save_indexing_system(sys).await
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(ServerFnError::new("Server operation not available in WASM"))
    }
}