//! Integration test for MongoDB sync functionality.
//!
//! Validates:
//!   1. MongoDB connection establishment
//!   2. Push (save) an `IndexingSystem` to MongoDB
//!   3. Pull (load) it back and verify the roundtrip
//!   4. Clean up by deleting the test document
//!
//! This test requires a running MongoDB instance at
//! `mongodb://localhost:<port>` (or the configured URI).
//! If MongoDB is not available the test is skipped gracefully.

use datawallet::indexing::IndexingSystem;
use datawallet::sync::{MongoConfig, MongoSync, SyncResult, MongoError};

/// Helper: build a small IndexingSystem with known entries for roundtrip verification.
fn create_test_system() -> IndexingSystem {
    let mut sys = IndexingSystem::new();
    sys.insert("001001", "Alice", "runs", "ball");
    sys.insert("002002", "Bob", "jumps", "car");
    sys.insert("003003", "Charlie", "throws", "disc");
    sys.reserve("009999");
    sys.capture_history("001001", "note", "Created Alice entry", None);
    sys.capture_history("002002", "note", "Created Bob entry", None);
    sys.register_registry("00", "Test registry A");
    sys.register_registry("01", "Test registry B");
    sys
}

/// Assert two IndexingSystems are deeply equal, with a descriptive message on mismatch.
fn assert_systems_equal(a: &IndexingSystem, b: &IndexingSystem, label: &str) {
    assert_eq!(
        a.len(),
        b.len(),
        "{label}: entry count mismatch ({} vs {})",
        a.len(),
        b.len()
    );
    assert_eq!(
        a.entries, b.entries,
        "{label}: entries mismatch"
    );
    assert_eq!(
        a.routing, b.routing,
        "{label}: routing mismatch"
    );
    assert_eq!(
        a.reservations, b.reservations,
        "{label}: reservations mismatch"
    );
    assert_eq!(
        a.registries, b.registries,
        "{label}: registries mismatch"
    );
    // Compare history length and content (timestamps may differ slightly)
    assert_eq!(
        a.history.len(),
        b.history.len(),
        "{label}: history length mismatch"
    );
    for (i, (ha, hb)) in a.history.iter().zip(b.history.iter()).enumerate() {
        assert_eq!(ha.index, hb.index, "{label}: history[{i}] index mismatch");
        assert_eq!(ha.entry_type, hb.entry_type, "{label}: history[{i}] entry_type mismatch");
        assert_eq!(ha.content, hb.content, "{label}: history[{i}] content mismatch");
    }
}

/// Clean up the test document from MongoDB after a successful test run.
fn cleanup(sync: &MongoSync) -> Result<(), MongoError> {
    #[cfg(feature = "server")]
    {
        let client = sync.config().clone();
        // Use a direct MongoDB client to delete our test doc
        let mongo_client = mongodb::sync::Client::with_uri_str(&client.uri)
            .map_err(|e| MongoError::Connection(e.to_string()))?;
        let collection = mongo_client
            .database(&client.database)
            .collection::<bson::Document>(&client.collection);
        let filter = bson::doc! { "_id": "pao-indexing-system" };
        collection
            .delete_one(filter)
            .run()
            .map_err(|e| MongoError::Write(e.to_string()))?;
        println!("[cleanup] Deleted test document from MongoDB");
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = sync;
    }
    Ok(())
}

// ── Core Test ─────────────────────────────────────────────────

#[cfg(feature = "server")]
#[test]
fn test_mongodb_connect_push_pull_roundtrip() {
    // 1. Connect
    let config = MongoConfig::default();
    let sync = match MongoSync::connect(config.clone()) {
        Ok(s) => {
            println!("✅ Connected to MongoDB at {}", config.uri);
            s
        }
        Err(e) => {
            println!("⚠️  MongoDB connection failed — is MongoDB running?\n   Error: {e}");
            println!("   Skipping test (not a failure).");
            return;
        }
    };

    // 2. Create test data and push
    let original = create_test_system();
    let push_result: SyncResult = sync
        .push(&original)
        .expect("Push to MongoDB should succeed");
    assert!(push_result.success, "Push result should indicate success");
    assert_eq!(push_result.entries_count, original.len());
    println!(
        "✅ Pushed {} entries to MongoDB: {}",
        push_result.entries_count, push_result.message
    );

    // 3. Pull it back
    let pulled = sync
        .pull()
        .expect("Pull from MongoDB should succeed")
        .expect("Pulled system should be Some (document exists)");
    println!("✅ Pulled {} entries from MongoDB", pulled.len());

    // 4. Verify roundtrip — the pulled data must match the original
    assert_systems_equal(&original, &pulled, "Roundtrip push→pull");

    // 5. Push an updated system
    let mut updated = original.clone();
    updated.insert("004004", "Diana", "catches", "frisbee");
    updated.capture_history("004004", "note", "Added Diana", None);
    sync.push(&updated).expect("Second push should succeed");
    println!("✅ Pushed updated system ({} entries)", updated.len());

    // 6. Pull and verify the update
    let pulled2 = sync
        .pull()
        .expect("Second pull should succeed")
        .expect("Second pulled system should be Some");
    assert_eq!(pulled2.len(), 4, "Updated system should have 4 entries");
    assert_eq!(
        pulled2.get("004004").map(|e| e.person.as_str()),
        Some("Diana"),
        "New entry should be Diana"
    );
    println!("✅ Verified updated data roundtrip");

    // 7. Clean up
    if let Err(e) = cleanup(&sync) {
        println!("⚠️  Cleanup warning (non-fatal): {e}");
    } else {
        println!("✅ Cleaned up test document");
    }

    println!("\n🎉 All MongoDB roundtrip assertions passed");
}

// ── Edge Cases ─────────────────────────────────────────────────

#[cfg(feature = "server")]
#[test]
fn test_mongodb_pull_empty_database() {
    let config = MongoConfig::default();
    let sync = match MongoSync::connect(config.clone()) {
        Ok(s) => s,
        Err(_) => {
            println!("⚠️  MongoDB not available — skipping empty-pull test");
            return;
        }
    };

    // Delete any existing document first
    let _ = cleanup(&sync);

    // Pull should return Ok(None) when no document exists
    let result = sync.pull();
    match result {
        Ok(None) => println!("✅ Pull from empty DB returned None as expected"),
        Ok(Some(sys)) => {
            panic!("Pull from empty DB should return None, got system with {} entries", sys.len());
        }
        Err(e) => {
            panic!("Pull from empty DB should succeed with Ok(None), got error: {e}");
        }
    }
}

#[cfg(feature = "server")]
#[test]
fn test_mongodb_sync_empty_local() {
    let config = MongoConfig::default();
    let sync = match MongoSync::connect(config.clone()) {
        Ok(s) => s,
        Err(_) => {
            println!("⚠️  MongoDB not available — skipping sync test");
            return;
        }
    };

    // Push a known system to MongoDB
    let remote_sys = create_test_system();
    sync.push(&remote_sys).expect("Push before sync test should succeed");

    // Sync with empty local should pull remote
    let empty_local = IndexingSystem::new();
    let merged = sync.sync(&empty_local).expect("Sync with empty local should succeed");
    assert_eq!(merged.len(), 3, "Sync with empty local should pull 3 entries");
    println!("✅ Sync with empty local pulled {} entries from remote", merged.len());

    // Clean up
    let _ = cleanup(&sync);
}

#[cfg(feature = "server")]
#[test]
fn test_mongodb_sync_empty_remote() {
    let config = MongoConfig::default();
    let sync = match MongoSync::connect(config.clone()) {
        Ok(s) => s,
        Err(_) => {
            println!("⚠️  MongoDB not available — skipping sync test");
            return;
        }
    };

    // Ensure remote is empty
    let _ = cleanup(&sync);

    // Sync with populated local should push to remote
    let local_sys = create_test_system();
    let merged = sync.sync(&local_sys).expect("Sync with populated local should succeed");
    assert_eq!(merged.len(), 3, "Sync should return local entries when remote is empty");
    println!("✅ Sync with empty remote pushed {} entries", merged.len());

    // Clean up
    let _ = cleanup(&sync);
}

// ── Disconnected / No-Server Stub Tests ────────────────────────

#[cfg(not(feature = "server"))]
#[test]
fn test_mongodb_not_available_without_server_feature() {
    let config = MongoConfig::default();
    let result = MongoSync::connect(config);
    match result {
        Err(MongoError::NotAvailable) => println!("✅ connect() correctly returned NotAvailable without server feature"),
        Err(e) => panic!("Expected MongoError::NotAvailable, got: {e}"),
        Ok(_) => panic!("Expected MongoError::NotAvailable, got Ok"),
    }
}

#[cfg(not(feature = "server"))]
#[test]
fn test_disconnected_stub_returns_not_available() {
    let config = MongoConfig::default();
    let sync = MongoSync::disconnected(config);
    let sys = IndexingSystem::new();

    let push_result = sync.push(&sys);
    assert!(matches!(push_result, Err(MongoError::NotAvailable)));

    let pull_result: Result<Option<IndexingSystem>, MongoError> = sync.pull();
    assert!(matches!(pull_result, Err(MongoError::NotAvailable)));

    let sync_result = sync.sync(&sys);
    assert!(matches!(sync_result, Err(MongoError::NotAvailable)));

    println!("✅ All stub methods return NotAvailable as expected");
}