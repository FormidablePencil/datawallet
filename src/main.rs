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
// server module is always compiled: on native it runs the actual file I/O,
// on wasm the #[server] macro emits HTTP-fetch stubs.
mod server;

// sync module contains #[server] functions (push_to_mongodb, pull_from_mongodb)
// that are only meaningful on the server side. On wasm they would emit
// wasm-bindgen JS imports that throw uncatchable errors.
// We exclude the entire module from wasm compilation.
#[cfg(not(target_arch = "wasm32"))]
mod sync;

use components::DataWalletApp;
use indexing::IndexingSystem;

fn main() {
    dioxus::launch(App);
}

/// Main application component
#[component]
fn App() -> Element {
    // ── Initialise indexing system ──────────────────────────
    //
    // use_server_future handles SSR→client synchronisation:
    //   - Server: runs server::load_indexing_system() which reads from disk
    //   - The result is serialised into the HTML and restored on the wasm
    //     client during hydration, avoiding VDOM mismatch.
    //
    // server::load_indexing_system() is a #[server] function, so on wasm
    // it becomes an HTTP-fetch stub (not a filesystem call). The fullstack
    // framework calls it during initial page load and embeds the result.
    let sys_future = use_server_future(move || server::load_indexing_system())?;

    let initial: IndexingSystem = match sys_future.read_unchecked().as_ref() {
        Some(Ok(sys)) => sys.clone(),
        _ => IndexingSystem::new(),
    };

    let mut indexing_system: Signal<IndexingSystem> = use_signal(|| initial.clone());
    let mut last_saved: Signal<IndexingSystem> = use_signal(|| initial);

    // When the future resolves, update the signal.
    // (During hydration on wasm, use_server_future already has the
    //  deserialised value, so this runs synchronously with correct data.)
    use_effect(move || {
        if let Some(Ok(sys)) = sys_future() {
            indexing_system.set(sys);
        }
    });

    // ── Auto-save: whenever indexing_system changes, persist via
    //    the server function (local file + MongoDB push).
    //    On WASM the #[server] call goes through HTTP to the backend.
    //    On native it runs the file I/O + MongoDB directly.
    //    Guards against re-saving unchanged state via `last_saved`.
    use_effect(move || {
        let current = indexing_system.read().clone();
        let previous = last_saved.read().clone();

        // Skip if nothing changed
        if current == previous {
            return;
        }

        // Update our saved version tracker
        last_saved.set(current.clone());

        spawn(async move {
            let _ = server::save_indexing_system(current).await;
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

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_add_hundred() {
        assert_eq!(add(100, 2), 102);
    }
}