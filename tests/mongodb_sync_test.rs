//! Integration test for MongoDB sync functionality

#[cfg(feature = "server")]
#[tokio::test]
async fn test_mongodb_connection() {
    use datawallet::sync::{MongoSync, MongoConfig};
    
    // Load config from file
    let config = MongoConfig::default();
    
    // Try to connect - this will fail if MongoDB is not running
    match MongoSync::connect(config) {
        Ok(sync) => {
            println!("✅ MongoDB connection successful");
            
            // Create a test system
            let test_system = datawallet::indexing::IndexingSystem::default();
            
            // Test push
            match sync.push(&test_system) {
                Ok(result) => {
                    println!("✅ Push successful: {}", result.message);
                }
                Err(e) => {
                    println!("❌ Push failed: {}", e);
                    panic!("Push failed: {}", e);
                }
            }
            
            // Test pull
            match sync.pull() {
                Ok(Some(system)) => {
                    println!("✅ Pull successful: {} entries", system.len());
                }
                Ok(None) => {
                    println!("⚠️ No data found in MongoDB");
                }
                Err(e) => {
                    println!("❌ Pull failed: {}", e);
                    panic!("Pull failed: {}", e);
                }
            }
            
            // Test sync
            match sync.sync(&test_system) {
                Ok(merged) => {
                    println!("✅ Sync successful: {} entries", merged.len());
                }
                Err(e) => {
                    println!("❌ Sync failed: {}", e);
                    panic!("Sync failed: {}", e);
                }
            }
        }
        Err(e) => {
            println!("⚠️ MongoDB connection failed (expected if not running): {}", e);
            // Don't panic - this is expected if MongoDB isn't running
        }
    }
}

#[cfg(feature = "server")]
#[tokio::test]
async fn test_server_functions() {
    use datawallet::sync::{push_to_mongodb, pull_from_mongodb};
    
    // Create a simple JSON representation
    let test_json = r#"{"entries":{},"history":[]}"#;
    
    // Test push via server function
    match push_to_mongodb(test_json.to_string()).await {
        Ok(result) => {
            println!("✅ push_to_mongodb() successful: {}", result);
        }
        Err(e) => {
            println!("⚠️ push_to_mongodb() failed (expected if server not running): {}", e);
        }
    }
    
    // Test pull via server function
    match pull_from_mongodb().await {
        Ok(Some(json)) => {
            println!("✅ pull_from_mongodb() successful: {}", &json[..std::cmp::min(100, json.len())]);
        }
        Ok(None) => {
            println!("⚠️ pull_from_mongodb() returned None (no data)");
        }
        Err(e) => {
            println!("⚠️ pull_from_mongodb() failed (expected if server not running): {}", e);
        }
    }
}