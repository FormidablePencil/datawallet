# Active Context: PaoValues

## Current Work Focus

**MongoDB document sync layer implemented for the IndexingSystem.**

A new `src/sync.rs` module provides a `MongoSync` client that stores the full `IndexingSystem` as a single MongoDB document (with `_id: "pao-indexing-system"`). This enables cloud-based persistence alongside the existing local file-based storage.

## Recent Changes

### MongoDB Sync Module (`src/sync.rs`)
- Created `MongoConfig` – connection URI, database, collection, timeout settings
- Created `MongoSync` struct with `connect()`, `push()`, `pull()`, `sync()` methods
- Created `SyncResult` and `MongoError` types for structured results
- `push()` – upserts the entire `IndexingSystem` to MongoDB as a BSON document
- `pull()` – reads the document from MongoDB and deserializes back to `IndexingSystem`
- `sync()` – two-way sync with heuristic (compares history timestamps) to pick the winner between local and remote
- Feature-gated behind `mongodb-sync` – compiles conditionally only when the feature is enabled
- When MongoDB is unavailable, gracefully degrades to local file-only mode

### Cargo.toml Changes
- Added `mongodb` (v3.2, sync) and `bson` (v2.13, chrono-0_4) as optional dependencies
- Added `mongodb-sync` feature flag under `[features]`
- `mongodb-sync` is included in the `default` feature set

### `src/main.rs` Changes
- Startup sync: on application launch, attempts to connect to MongoDB and sync
- If MongoDB is available, loads local system, syncs with remote, saves the merged result
- If MongoDB is not available, logs a message and continues with local file storage

### `src/indexing.rs` Changes
- Added `PartialEq` derives to `RegistryMeta` and `IndexingSystem` (needed for comparison in main.rs sync logic)

## Next Steps

1. Install a C compiler (MSVC or MinGW) on the build machine so the full `mongodb-sync` feature can be compiled
2. Add a UI component in the DataWallet app to trigger manual sync (push/pull)
3. Consider adding a `MongoConfig` JSON file loader so users can configure the connection URI without recompiling
4. Add unit tests for the sync module

## Active Decisions and Considerations

### Single Document Design
- The entire `IndexingSystem` is stored as ONE MongoDB document rather than individual entries
- **Rationale**: The dataset is small (< 1000 entries), and this keeps atomicity simple
- **Trade-off**: Not suitable for massive datasets, but perfectly fine for a personal PAO system

### Sync Heuristic
- Uses the last history entry's timestamp to decide which side is newer
- If local is empty and remote has data → pull
- If remote is newer → pull
- If local is current → push
- **Limitation**: No merge conflict resolution yet. The newer system completely replaces the older one.

### Graceful Degradation
- If MongoDB is not running, the application still works with local files
- No startup failure if connection fails

## Important Patterns and Preferences

### Feature Flag
```toml
[features]
mongodb-sync = ["mongodb", "bson"]
```
All MongoDB-specific code is `#[cfg(feature = "mongodb-sync")]` guarded. When the feature is off, the types still exist but return `MongoError::NotAvailable`.

### Document Structure in MongoDB
```
{
  "_id": "pao-indexing-system",
  "entries": { ... },
  "routing": { ... },
  "history": [ ... ],
  "reservations": [ ... ],
  "registries": { ... },
  "inverted": { ... }
}
```

## Learnings and Project Insights

- The `ring` crate (required by MongoDB driver) needs a C compiler (MSVC or GCC). The GNU toolchain was default but no GCC was installed. Switched back to GNU and built without mongodb-sync for compilation check.
- The sync module design is clean and non-intrusive: existing data structures didn't need modification, only new code was added.