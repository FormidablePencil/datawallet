//! (dynamic) DataWallet - A dynamic data registry and mnemonic PAO indexing system
//!
//! This application provides a 6-digit (XX-XX-XX) registry system for PAO entries,
//! with 4-quadrant letter mapping, phonetic encoding, reservation/intellisense,
//! and branching support.

#![allow(non_snake_case)]

use dioxus::prelude::*;

mod components;
mod data;
mod hooks;
mod indexing;
mod sync;

use components::DataWalletApp;

fn main() {
    dioxus::launch(App);
}

/// Main application component
#[component]
fn App() -> Element {
    // Load indexing system from local file or create default
    let mut indexing_system: Signal<indexing::IndexingSystem> = use_signal(|| {
        indexing::IndexingSystem::load_or_default()
    });

    // Run MongoDB sync on mount
    use_effect(move || {
        spawn(async move {
            let local = indexing_system.read().clone();
            
            // Try to pull remote state
            if let Ok(Some(remote_str)) = sync::pull_from_mongodb().await {
                if let Ok(remote) = serde_json::from_str::<indexing::IndexingSystem>(&remote_str) {
                    if local.len() == 0 && remote.len() > 0 {
                        // Local is empty but remote has data → use remote
                        println!("[MongoSync] Pulled {} entries from MongoDB", remote.len());
                        indexing_system.set(remote);
                    } else if sync::remote_synced_after_local(&local, &remote) {
                        // Remote is newer → pull
                        println!("[MongoSync] Remote is newer – pulled {} entries", remote.len());
                        indexing_system.set(remote);
                    } else {
                        // Local is current → push
                        if let Ok(json) = serde_json::to_string(&local) {
                            let _ = sync::push_to_mongodb(json).await;
                            println!("[MongoSync] Pushed local to MongoDB");
                        }
                    }
                }
            } else if local.len() > 0 {
                // If pull failed or returned None, but we have local data, try to push it
                if let Ok(json) = serde_json::to_string(&local) {
                    let _ = sync::push_to_mongodb(json).await;
                    println!("[MongoSync] Pushed local to MongoDB (initial setup)");
                }
            }
        });
    });

    rsx! {
        // Link the Tailwind CSS stylesheet generated from input.css
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        div { class: "min-h-screen bg-gradient-to-br from-gray-900 via-indigo-900 to-gray-900",
            DataWalletApp {
                indexing_system: indexing_system,
            }
        }
    }
}
