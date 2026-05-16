//! DataWallet UI Components
//!
//! Provides the main DataWalletApp component with full registry management,
//! 6-digit index entry, 4 display modes, reservation/intellisense, and history.

use dioxus::prelude::*;
use crate::indexing::{self, IndexingSystem, RegistryIndex, HistoryEntry};

/// Helper: compute all display-mode previews for an entry for the browse list.
fn entry_previews(idx: &str) -> String {
    let ri = RegistryIndex::from_6digit(idx);
    format!(
        "📖 {}  🔢 {}  🎤 {}",
        ri.to_letters(),
        ri.to_numbers(),
        crate::indexing::phonetic_word(idx),
    )
}

/// Helper: render index in the active display mode.
fn rendered_index(idx: &str, display_mode: &str, active_idx: &str) -> String {
    if !active_idx.is_empty() && idx == active_idx {
        let ri = RegistryIndex::from_6digit(idx);
        match display_mode {
            "letters" => ri.to_letters(),
            "numbers" => ri.to_numbers(),
            "alt_lnl" => ri.to_alternating(true),
            "alt_nln" => ri.to_alternating(false),
            "phonetic" => ri.to_phonetic_display(&[]),
            _ => ri.to_numbers(),
        }
    } else {
        idx.to_string()
    }
}

/// Main DataWallet application component
#[component]
pub fn DataWalletApp(
    indexing_system: Signal<IndexingSystem>,
) -> Element {
    // ── Form state ──
    let mut registry_input = use_signal(|| String::from("00"));
    let mut index_input = use_signal(|| String::from("00"));
    let mut closing_input = use_signal(|| String::from("00"));
    let mut person_input = use_signal(|| String::new());
    let mut action_input = use_signal(|| String::new());
    let mut object_input = use_signal(|| String::new());
    let mut note_input = use_signal(|| String::new());
    let mut search_query = use_signal(|| String::new());
    let mut status_msg = use_signal(|| String::new());
    let mut display_mode = use_signal(|| String::from("numbers"));
    let mut reserve_input = use_signal(|| String::new());
    let mut route_from = use_signal(|| String::new());
    let mut route_to = use_signal(|| String::new());
    let mut active_tab = use_signal(|| String::from("entries"));
    let mut export_path = use_signal(|| String::from("data/exports/"));
    let mut import_path = use_signal(|| String::from("data/"));
    let mut show_export_import = use_signal(|| false);

    // ── Pre-computed values ──
    let entries = indexing_system.read().all_entries().iter().map(|e| (*e).clone()).collect::<Vec<_>>();

    let registries: Vec<RegistryCard> = indexing_system.read().all_registries().iter().map(|m| {
        let ri = RegistryIndex::from_6digit(&format!("{}0000", m.prefix));
        RegistryCard {
            prefix: m.prefix.clone(),
            desc: m.description.clone(),
            quad: m.quadrant,
            letters: ri.to_letters(),
            created: m.created.clone(),
        }
    }).collect();

    let reservation_pairs: Vec<(String, String)> = indexing_system.read().reservations.iter().map(|idx| {
        let ri = RegistryIndex::from_6digit(idx);
        (idx.clone(), ri.to_letters())
    }).collect();

    let inverted_pairs: Vec<(String, String)> = indexing_system.read().all_inverted().iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    let routes_snapshot: Vec<RouteRow> = indexing_system.read().routing.iter()
        .map(|(k, v)| {
            let rtype = if k.ends_with(":branch") { "Branch" } else if k.ends_with("-inv") { "Inverted" } else { "Route" };
            RouteRow { from: k.to_string(), to: v.to_string(), rtype: rtype.to_string() }
        })
        .collect();

    let full_index: String = registry_input()
        .chars().chain(index_input().chars()).chain(closing_input().chars())
        .filter(|c| c.is_ascii_digit()).collect();

    let phonetic_preview_str = if full_index.len() >= 2 { indexing::phonetic_word(&full_index) } else { String::new() };

    let ri_display_str = if full_index.len() >= 6 {
        let ri = RegistryIndex::from_6digit(&full_index);
        match display_mode().as_str() {
            "letters" => ri.to_letters(),
            "numbers" => ri.to_numbers(),
            "alt_lnl" => ri.to_alternating(true),
            "alt_nln" => ri.to_alternating(false),
            "phonetic" => ri.to_phonetic_display(&[]),
            _ => ri.to_numbers(),
        }
    } else { String::new() };

    let conflict_msgs: Vec<String> = if full_index.len() >= 6 { indexing_system.read().check_conflicts(&full_index) } else { Vec::new() };
    let suggestion_list: Vec<String> = if full_index.len() >= 6 { indexing_system.read().suggest_safe_indices(&full_index) } else { Vec::new() };
    let quadrant_label: String = if !registry_input().is_empty() {
        let reg_num = registry_input().parse::<u32>().unwrap_or(0);
        indexing::quadrant_label(reg_num).to_string()
    } else { String::new() };

    let has_preview = full_index.len() >= 2;
    let has_full_index = full_index.len() >= 6;

    // ── Filtered entry vnodes (browse entries search) ──
    let search_text = search_query();
    let query = search_text.to_lowercase();
    let dm = display_mode();
    let fi = full_index.clone();
    let filtered_entry_vnodes: Vec<_> = entries.iter()
        .filter(|e| {
            query.is_empty()
                || e.index.contains(&query)
                || e.person.to_lowercase().contains(&query)
                || e.action.to_lowercase().contains(&query)
                || e.object.to_lowercase().contains(&query)
        })
        .map(|e| {
            let idx_display = rendered_index(&e.index, &dm, &fi);
            let previews = entry_previews(&e.index);
            let phonetic = crate::indexing::phonetic_word(&e.index);
            let e_idx = e.index.clone();
            let e_person = e.person.clone();
            let e_action = e.action.clone();
            let e_object = e.object.clone();
            rsx! {
                div { class: "p-4 bg-gray-700 rounded-lg border-l-4 border-l-indigo-500 hover:bg-gray-700/80 transition-colors",
                    div { class: "flex justify-between items-start",
                        div { class: "flex items-center gap-3",
                            span { class: "font-mono text-lg text-indigo-400 font-bold", "{idx_display}" }
                            span { class: "text-xs text-gray-400 font-mono", "phonetic: {phonetic}" }
                        }
                        button {
                            class: "text-xs text-red-400 hover:text-red-300 transition-colors",
                            onclick: {
                                let ei = e_idx.clone();
                                move |_| { indexing_system.write().remove(&ei); let _ = indexing_system.read().save(); status_msg.set(format!("🗑️ Removed: {}", ei)); }
                            },
                            "✕"
                        }
                    }
                    p { class: "text-sm text-white mt-2",
                        strong { "{e_person} " }
                        span { class: "text-gray-300", "{e_action} {e_object}" }
                    }
                    div { class: "mt-2 text-xs text-gray-500", "{previews}" }
                }
            }
        })
        .collect();

    // ── History filtering ──
    let history_entries: Vec<HistoryEntry> = {
        let sys = indexing_system.read();
        sys.history().into_iter().cloned().collect()
    };
    let history_filtered: Vec<_> = if search_query().is_empty() {
        history_entries.iter().rev().map(|entry| {
            let idx = entry.index.clone();
            let ts = entry.timestamp.clone();
            let content = entry.content.clone();
            let etype = entry.entry_type.clone();
            let ctx = entry.context.clone();
            let icon = match etype.as_str() {
                "note" => "📝", "screenshot" => "📸", "tooltip" => "💡", "highlight" => "📌", _ => "📄",
            };
            rsx! {
                div { class: "p-3 bg-gray-700 rounded-lg border-l-4 border-l-blue-500",
                    div { class: "flex justify-between items-start",
                        span { class: "font-mono text-sm text-blue-400 font-bold", "{idx}" }
                        span { class: "text-xs text-gray-500", "{icon} {ts}" }
                    }
                    p { class: "text-sm text-gray-300 mt-1", "{content}" }
                    if let Some(c) = ctx {
                        p { class: "text-xs text-gray-500 mt-1 italic", "{c}" }
                    }
                }
            }
        }).collect()
    } else {
        let q = search_query().to_lowercase();
        history_entries.iter().rev()
            .filter(|entry| {
                entry.index.to_lowercase().contains(&q)
                    || entry.content.to_lowercase().contains(&q)
                    || entry.entry_type.to_lowercase().contains(&q)
                    || entry.context.as_ref().map_or(false, |c| c.to_lowercase().contains(&q))
            })
            .map(|entry| {
                let idx = entry.index.clone();
                let ts = entry.timestamp.clone();
                let content = entry.content.clone();
                let etype = entry.entry_type.clone();
                let ctx = entry.context.clone();
                let icon = match etype.as_str() {
                    "note" => "📝", "screenshot" => "📸", "tooltip" => "💡", "highlight" => "📌", _ => "📄",
                };
                rsx! {
                    div { class: "p-3 bg-gray-700 rounded-lg border-l-4 border-l-blue-500",
                        div { class: "flex justify-between items-start",
                            span { class: "font-mono text-sm text-blue-400 font-bold", "{idx}" }
                            span { class: "text-xs text-gray-500", "{icon} {ts}" }
                        }
                        p { class: "text-sm text-gray-300 mt-1", "{content}" }
                        if let Some(c) = ctx {
                            p { class: "text-xs text-gray-500 mt-1 italic", "{c}" }
                        }
                    }
                }
            }).collect()
    };

    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-gray-900 via-indigo-900 to-gray-900 text-white",
            header { class: "bg-gray-800 border-b border-indigo-500 shadow-lg px-6 py-4",
                div { class: "container mx-auto flex justify-between items-center",
                    div {
                        h1 { class: "text-3xl font-bold bg-gradient-to-r from-indigo-400 to-purple-400 bg-clip-text text-transparent",
                            "(dynamic) DataWallet"
                        }
                        p { class: "text-sm text-gray-400 mt-1",
                            "6-digit PAO registry system with phonetic encoding & 4-quadrant mapping"
                        }
                        div { class: "mt-2 flex gap-2",
                            button {
                                class: "px-2 py-1 text-xs bg-green-600 hover:bg-green-500 text-white rounded transition-colors",
                                onclick: move |_| {
                                    if let Ok(json) = indexing_system.read().export_to_json() {
                                        let path = format!("data/exports/indexing_export.json");
                                        if let Some(parent) = std::path::Path::new(&path).parent() {
                                            let _ = std::fs::create_dir_all(parent);
                                        }
                                        if std::fs::write(&path, &json).is_ok() {
                                            status_msg.set(format!("✅ Exported to {path}"));
                                        } else {
                                            status_msg.set("❌ Export failed".to_string());
                                        }
                                    }
                                },
                                "⬇️ Export JSON"
                            }
                            if show_export_import() {
                                div { class: "flex gap-2 items-center",
                                    input { class: "px-2 py-1 text-xs bg-gray-700 text-white rounded border border-gray-600 w-48",
                                        placeholder: "data/values/indexing_store.json",
                                        value: "{import_path}",
                                        oninput: move |e| import_path.set(e.value()),
                                    }
                                    button {
                                        class: "px-2 py-1 text-xs bg-red-600 hover:bg-red-500 text-white rounded transition-colors",
                                        onclick: move |_| {
                                            let path = import_path();
                                            if let Ok(contents) = std::fs::read_to_string(&path) {
                                                match IndexingSystem::import_from_json(&contents) {
                                                    Ok(imported) => {
                                                        *indexing_system.write() = imported;
                                                        let _ = indexing_system.read().save();
                                                        status_msg.set(format!("✅ Imported from {path}"));
                                                    }
                                                    Err(e) => status_msg.set(format!("❌ Import failed: {e}")),
                                                }
                                            } else {
                                                status_msg.set(format!("❌ Failed to read {path}"));
                                            }
                                        },
                                        "⬆️ Import"
                                    }
                                    button {
                                        class: "px-2 py-1 text-xs bg-gray-600 hover:bg-gray-500 text-white rounded transition-colors",
                                        onclick: move |_| show_export_import.set(false),
                                        "✕"
                                    }
                                }
                            } else {
                                button {
                                    class: "px-2 py-1 text-xs bg-blue-600 hover:bg-blue-500 text-white rounded transition-colors",
                                    onclick: move |_| show_export_import.set(true),
                                    "⚙️ Import"
                                }
                            }
                        }
                    }
                    div { class: "text-xs text-gray-500",
                        "Entries: {entries.len()} | Registries: {registries.len()} | Reserved: {reservation_pairs.len()}"
                    }
                }
            }

            div { class: "container mx-auto px-4 py-6",
                div { class: "flex space-x-2 mb-6 border-b border-gray-700",
                    button {
                        class: "px-4 py-2 rounded-t-lg font-medium transition-colors",
                        class: if active_tab() == "entries" { "bg-gray-800 text-indigo-400 border-t-2 border-indigo-400" } else { "text-gray-400 hover:text-white hover:bg-gray-800/50" },
                        onclick: move |_| active_tab.set("entries".to_string()),
                        "📝 Entries"
                    }
                    button {
                        class: "px-4 py-2 rounded-t-lg font-medium transition-colors",
                        class: if active_tab() == "registries" { "bg-gray-800 text-indigo-400 border-t-2 border-indigo-400" } else { "text-gray-400 hover:text-white hover:bg-gray-800/50" },
                        onclick: move |_| active_tab.set("registries".to_string()),
                        "🏛️ Registries"
                    }
                    button {
                        class: "px-4 py-2 rounded-t-lg font-medium transition-colors",
                        class: if active_tab() == "reservations" { "bg-gray-800 text-indigo-400 border-t-2 border-indigo-400" } else { "text-gray-400 hover:text-white hover:bg-gray-800/50" },
                        onclick: move |_| active_tab.set("reservations".to_string()),
                        "🔒 Reservations"
                    }
                    button {
                        class: "px-4 py-2 rounded-t-lg font-medium transition-colors",
                        class: if active_tab() == "history" { "bg-gray-800 text-indigo-400 border-t-2 border-indigo-400" } else { "text-gray-400 hover:text-white hover:bg-gray-800/50" },
                        onclick: move |_| active_tab.set("history".to_string()),
                        "📜 History"
                    }
                    button {
                        class: "px-4 py-2 rounded-t-lg font-medium transition-colors",
                        class: if active_tab() == "routing" { "bg-gray-800 text-indigo-400 border-t-2 border-indigo-400" } else { "text-gray-400 hover:text-white hover:bg-gray-800/50" },
                        onclick: move |_| active_tab.set("routing".to_string()),
                        "🔀 Routing"
                    }
                }

                // Entries Tab
                if active_tab() == "entries" {
                    div { class: "mb-6 p-5 bg-gray-800 rounded-lg border border-indigo-500/50",
                        h2 { class: "text-xl font-bold text-indigo-400 mb-4", "🧩 6-Digit Index Builder" }
                        div { class: "grid grid-cols-1 md:grid-cols-6 gap-3 mb-4",
                            div { class: "md:col-span-2",
                                label { class: "text-xs text-gray-400 mb-1 block", "Registry" }
                                input { class: "w-full px-3 py-2 bg-gray-700 text-white rounded border border-gray-600 focus:outline-none focus:ring-2 focus:ring-indigo-500 font-mono text-center",
                                    placeholder: "00", maxlength: "2", value: "{registry_input}",
                                    oninput: move |e| registry_input.set(e.value().chars().filter(|c| c.is_ascii_digit()).take(2).collect()),
                                }
                            }
                            div { class: "md:col-span-2",
                                label { class: "text-xs text-gray-400 mb-1 block", "Index" }
                                input { class: "w-full px-3 py-2 bg-gray-700 text-white rounded border border-gray-600 focus:outline-none focus:ring-2 focus:ring-indigo-500 font-mono text-center",
                                    placeholder: "00", maxlength: "2", value: "{index_input}",
                                    oninput: move |e| index_input.set(e.value().chars().filter(|c| c.is_ascii_digit()).take(2).collect()),
                                }
                            }
                            div { class: "md:col-span-2",
                                label { class: "text-xs text-gray-400 mb-1 block", "Closing" }
                                input { class: "w-full px-3 py-2 bg-gray-700 text-white rounded border border-gray-600 focus:outline-none focus:ring-2 focus:ring-indigo-500 font-mono text-center",
                                    placeholder: "00", maxlength: "2", value: "{closing_input}",
                                    oninput: move |e| closing_input.set(e.value().chars().filter(|c| c.is_ascii_digit()).take(2).collect()),
                                }
                            }
                        }
                        if has_preview {
                            div { class: "p-4 bg-gray-900/50 rounded-lg mb-4 border border-gray-700",
                                div { class: "flex flex-wrap items-center gap-4",
                                    div { class: "text-center",
                                        p { class: "text-xs text-gray-500 mb-1", "Full Index" }
                                        p { class: "font-mono text-2xl font-bold text-white", "{full_index}" }
                                    }
                                    div { class: "text-center",
                                        p { class: "text-xs text-gray-500 mb-1", "Phonetic" }
                                        p { class: "font-mono text-lg text-purple-400", "{phonetic_preview_str}" }
                                    }
                                    if has_full_index {
                                        div { class: "text-center",
                                            p { class: "text-xs text-gray-500 mb-1", "Display" }
                                            p { class: "font-mono text-lg text-indigo-400", "{ri_display_str}" }
                                        }
                                    }
                                    div { class: "ml-auto",
                                        select {
                                            class: "px-3 py-2 bg-gray-700 text-white rounded border border-gray-600 focus:outline-none focus:ring-2 focus:ring-indigo-500",
                                            value: "{display_mode}",
                                            onchange: move |e| display_mode.set(e.value()),
                                            option { value: "numbers", "🔢 Numbers" }
                                            option { value: "letters", "🔤 Letters" }
                                            option { value: "alt_lnl", "⏮ L-N-L" }
                                            option { value: "alt_nln", "⏭ N-L-N" }
                                            option { value: "phonetic", "🎤 Phonetic" }
                                        }
                                    }
                                }
                                if !quadrant_label.is_empty() {
                                    p { class: "text-xs text-gray-500 mt-2", "{quadrant_label}" }
                                }
                            }
                        }
                        if !conflict_msgs.is_empty() {
                            div { class: "mb-4 p-3 bg-red-900/30 border border-red-500/50 rounded-lg",
                                p { class: "text-sm font-semibold text-red-400 mb-2", "⚠️ Conflicts" }
                                for c in &conflict_msgs { p { class: "text-xs text-red-300 ml-4", "• {c}" } }
                            }
                        }
                        if !suggestion_list.is_empty() {
                            div { class: "mb-4 p-3 bg-green-900/30 border border-green-500/50 rounded-lg",
                                p { class: "text-sm font-semibold text-green-400 mb-2", "💡 Suggestions" }
                                for s in &suggestion_list { p { class: "text-xs text-green-300 ml-4 font-mono", "• {s}" } }
                            }
                        }
                        div { class: "grid grid-cols-1 md:grid-cols-4 gap-3 mb-3",
                            input { class: "px-3 py-2 bg-gray-700 text-white rounded border border-gray-600 focus:outline-none focus:ring-2 focus:ring-purple-500", placeholder: "Person", value: "{person_input}", oninput: move |e| person_input.set(e.value()) }
                            input { class: "px-3 py-2 bg-gray-700 text-white rounded border border-gray-600 focus:outline-none focus:ring-2 focus:ring-purple-500", placeholder: "Action", value: "{action_input}", oninput: move |e| action_input.set(e.value()) }
                            input { class: "px-3 py-2 bg-gray-700 text-white rounded border border-gray-600 focus:outline-none focus:ring-2 focus:ring-purple-500", placeholder: "Object", value: "{object_input}", oninput: move |e| object_input.set(e.value()) }
                            input { class: "px-3 py-2 bg-gray-700 text-white rounded border border-gray-600 focus:outline-none focus:ring-2 focus:ring-purple-500", placeholder: "Note...", value: "{note_input}", oninput: move |e| note_input.set(e.value()) }
                        }
                        div { class: "flex gap-3",
                            button {
                                class: "px-6 py-2 bg-indigo-600 hover:bg-indigo-500 text-white rounded-lg font-semibold transition-colors",
                                onclick: move |_| {
                                    let idx = registry_input().chars().chain(index_input().chars()).chain(closing_input().chars()).filter(|c| c.is_ascii_digit()).collect::<String>();
                                    let person = person_input(); let action = action_input(); let object = object_input();
                                    if idx.len() < 2 || person.is_empty() { status_msg.set("Index and Person are required.".to_string()); return; }
                                    let c = indexing_system.read().check_conflicts(&idx);
                                    if !c.is_empty() { status_msg.set(format!("Cannot save: {}", c[0])); return; }
                                    indexing_system.write().insert(&idx, &person, &action, &object);
                                    if !note_input().is_empty() { indexing_system.write().capture_history(&idx, "note", &note_input(), None); }
                                    if let Err(e) = indexing_system.read().save() { status_msg.set(format!("Save error: {e}")); }
                                    else { status_msg.set(format!("✅ Saved: {idx} – {person}")); }
                                    person_input.set(String::new()); action_input.set(String::new());
                                    object_input.set(String::new()); note_input.set(String::new());
                                },
                                "💾 Save Entry"
                            }
                            button {
                                class: "px-4 py-2 bg-yellow-600 hover:bg-yellow-500 text-white rounded-lg font-semibold transition-colors",
                                onclick: move |_| {
                                    let idx = registry_input().chars().chain(index_input().chars()).chain(closing_input().chars()).filter(|c| c.is_ascii_digit()).collect::<String>();
                                    if idx.len() >= 2 {
                                        let reserved = indexing_system.write().reserve(&idx);
                                        status_msg.set(if reserved { format!("🔒 Reserved: {idx}") } else { format!("Already reserved: {idx}") });
                                    }
                                },
                                "🔒 Reserve"
                            }
                        }
                    }

                    div { class: "mb-6 p-5 bg-gray-800 rounded-lg border border-gray-700",
                        h2 { class: "text-xl font-bold text-green-400 mb-4", "🔍 Browse Entries" }
                        input { class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border border-gray-600 focus:outline-none focus:ring-2 focus:ring-green-500 mb-4",
                            placeholder: "Search...", value: "{search_query}",
                            oninput: move |e| search_query.set(e.value()),
                        }
                        div { class: "max-h-96 overflow-y-auto space-y-2",
                            if filtered_entry_vnodes.is_empty() {
                                p { class: "text-gray-400 text-sm italic py-8 text-center", "No entries found." }
                            } else {
                                for vnode in &filtered_entry_vnodes {
                                    {vnode}
                                }
                            }
                        }
                    }

                    if !status_msg().is_empty() {
                        div { class: "mb-4 p-3 bg-indigo-900/30 border border-indigo-500/50 rounded-lg",
                            p { class: "text-sm text-indigo-300", "{status_msg}" }
                        }
                    }
                }

                // Registries Tab
                if active_tab() == "registries" {
                    div { class: "p-5 bg-gray-800 rounded-lg border border-gray-700",
                        h2 { class: "text-xl font-bold text-indigo-400 mb-4", "🏛️ Registries" }
                        if registries.is_empty() {
                            p { class: "text-gray-400 text-sm italic", "No registries registered." }
                        } else {
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                for r in &registries {
                                    div { class: "p-4 bg-gray-700 rounded-lg border border-gray-600",
                                        p { class: "text-lg font-bold text-indigo-400 font-mono", "{r.prefix}" }
                                        p { class: "text-sm text-gray-300 mt-1", "{r.desc}" }
                                        p { class: "text-xs text-gray-500 mt-1", "Quadrant {r.quad}: {r.letters}" }
                                        p { class: "text-xs text-gray-500", "Created: {r.created}" }
                                    }
                                }
                            }
                        }
                    }
                }

                // Reservations Tab
                if active_tab() == "reservations" {
                    div { class: "p-5 bg-gray-800 rounded-lg border border-gray-700",
                        h2 { class: "text-xl font-bold text-yellow-400 mb-4", "🔒 Reservations & Intellisense" }
                        div { class: "flex gap-3 mb-6",
                            input { class: "flex-1 px-3 py-2 bg-gray-700 text-white rounded border border-gray-600 focus:outline-none focus:ring-2 focus:ring-yellow-500 font-mono",
                                placeholder: "6-digit index (e.g. 001234)", value: "{reserve_input}",
                                oninput: move |e| reserve_input.set(e.value()),
                            }
                            button {
                                class: "px-4 py-2 bg-yellow-600 hover:bg-yellow-500 text-white rounded-lg transition-colors",
                                onclick: move |_| {
                                    let idx = reserve_input();
                                    if idx.len() >= 2 {
                                        let c = indexing_system.read().check_conflicts(&idx);
                                        if !c.is_empty() { status_msg.set(format!("Conflict: {}", c[0])); }
                                        else {
                                            let reserved = indexing_system.write().reserve(&idx);
                                            status_msg.set(if reserved { format!("🔒 Reserved: {idx}") } else { format!("Already reserved: {idx}") });
                                        }
                                        reserve_input.set(String::new());
                                    }
                                },
                                "🔒 Reserve"
                            }
                        }
                        if reservation_pairs.is_empty() {
                            p { class: "text-gray-400 text-sm italic", "No reservations yet." }
                        } else {
                            div { class: "space-y-2",
                                for (idx_str, letters) in &reservation_pairs {
                                    div { class: "flex items-center justify-between p-3 bg-gray-700 rounded-lg border border-yellow-600/50",
                                        div { class: "flex items-center gap-3",
                                            span { class: "font-mono text-yellow-400 font-bold", "{idx_str}" }
                                            span { class: "text-xs text-gray-400", "{letters}" }
                                        }
                                        button {
                                            class: "text-xs text-gray-400 hover:text-red-400 transition-colors",
                                            onclick: {
                                                let owned = idx_str.clone();
                                                move |_| { indexing_system.write().unreserve(&owned); status_msg.set(format!("🔓 Unreserved: {owned}")); }
                                            },
                                            "🔓 Unreserve"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // History Tab
                if active_tab() == "history" {
                    div { class: "p-5 bg-gray-800 rounded-lg border border-gray-700",
                        h2 { class: "text-xl font-bold text-blue-400 mb-4", "📜 History" }
                        input { class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border border-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500 mb-4",
                            placeholder: "Search history (fuzzy)...", value: "{search_query}",
                            oninput: move |e| search_query.set(e.value()),
                        }
                        if history_filtered.is_empty() {
                            p { class: "text-gray-400 text-sm italic", "No history entries found." }
                        } else {
                            div { class: "space-y-2 max-h-96 overflow-y-auto",
                                for vnode in &history_filtered {
                                    {vnode}
                                }
                            }
                        }
                    }
                }

                // Routing Tab
                if active_tab() == "routing" {
                    div { class: "p-5 bg-gray-800 rounded-lg border border-gray-700",
                        h2 { class: "text-xl font-bold text-purple-400 mb-4", "🔀 Routing & Branching" }
                        if !routes_snapshot.is_empty() {
                            h3 { class: "text-lg font-semibold text-white mb-3", "Active Routes" }
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-3 mb-6",
                                for r in &routes_snapshot {
                                    div { class: "p-3 bg-gray-700 rounded-lg border border-gray-600",
                                        p { class: "text-xs text-gray-400", "{r.rtype}" }
                                        p { class: "font-mono text-sm text-purple-400", "{r.from} → {r.to}" }
                                    }
                                }
                            }
                        }
                        h3 { class: "text-lg font-semibold text-white mb-3", "Add Branch Route" }
                        div { class: "flex gap-3 mb-6",
                            input { class: "flex-1 px-3 py-2 bg-gray-700 text-white rounded border border-gray-600 focus:outline-none focus:ring-2 focus:ring-purple-500 font-mono text-center",
                                placeholder: "From (e.g. 00)", value: "{route_from}",
                                oninput: move |e| route_from.set(e.value()),
                            }
                            span { class: "self-center text-gray-400 text-lg", "→" }
                            input { class: "flex-1 px-3 py-2 bg-gray-700 text-white rounded border border-gray-600 focus:outline-none focus:ring-2 focus:ring-purple-500 font-mono text-center",
                                placeholder: "To (e.g. 24)", value: "{route_to}",
                                oninput: move |e| route_to.set(e.value()),
                            }
                            button {
                                class: "px-4 py-2 bg-purple-600 hover:bg-purple-500 text-white rounded-lg transition-colors",
                                onclick: move |_| {
                                    let from = route_from(); let to = route_to();
                                    if !from.is_empty() && !to.is_empty() {
                                        indexing_system.write().add_route(&format!("{}:branch", from), &to);
                                        let _ = indexing_system.read().save();
                                        status_msg.set(format!("✅ Branch: {from} → {to}"));
                                        route_from.set(String::new()); route_to.set(String::new());
                                    }
                                },
                                "Add Branch"
                            }
                        }
                        if !inverted_pairs.is_empty() {
                            h3 { class: "text-lg font-semibold text-white mb-3", "Inverted Indices" }
                            div { class: "space-y-2 mb-4",
                                for (k, v) in &inverted_pairs {
                                    div { class: "p-2 bg-gray-700 rounded-lg flex justify-between items-center",
                                        span { class: "font-mono text-sm text-orange-400", "{k} → {v}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// A pre-computed registry card for rendering.
struct RegistryCard {
    prefix: String,
    desc: String,
    quad: u8,
    letters: String,
    created: String,
}

/// A pre-computed route row.
struct RouteRow {
    from: String,
    to: String,
    rtype: String,
}