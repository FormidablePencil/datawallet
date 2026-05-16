//! DataWallet – dynamic index registry system
//!
//! This module implements a 6-digit (XX-XX-XX) registry system for PAO (Person-Action-Object)
//! mnemonic entries. It supports:
//!   1. Corrected Major System phonetic mapping (0/5=s/z,l; 1/6=t/d,j/sh/ch; 2/7=n,k/g; 3/8=m,f/v; 4/9=r,p/b).
//!   2. Folding rule: when both digits of a 2-digit pair are the same digit, the second digit
//!      "falls back" to the paired group's phonetic, delimited by "o".
//!   3. 6-digit index structure: [Registry 2-digit][Index 2-digit][Closing 2-digit].
//!   4. Four quadrant 25-value letter mapping (00-24:a→y, 25-49:-y→-a, 50-74:-A→-Y, 75-99:Y→A).
//!   5. Branching: direct association to indices from another registry, or rerouted to inverted indices.
//!   6. Four display modes: letters, numbers, alternating, phonetic.
//!   7. Reservation & intellisense: detect conflicts and symmetrical patterns.
//!   8. File-based persistence to JSON.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

// ── Corrected Major System Phonetic Mapping ──────────────────
//
// Digit pairs (folding pairs):
//   0 <-> 5    1 <-> 6    2 <-> 7    3 <-> 8    4 <-> 9
//
//   0 = s,z     5 = l
//   1 = t,d     6 = j,sh,ch
//   2 = n       7 = k,g
//   3 = m       8 = f,v
//   4 = r       9 = p,b

/// The full phonetic mapping: (digit_label, paired_digit_label, &primary_consonants, &paired_consonants).
pub const PHONETIC_PAIRS: [(&str, &str, &[&str], &[&str]); 5] = [
    ("0", "5", &["s", "z"], &["l"]),
    ("1", "6", &["t", "d"], &["j", "sh", "ch"]),
    ("2", "7", &["n"],       &["k", "g"]),
    ("3", "8", &["m"],       &["f", "v"]),
    ("4", "9", &["r"],       &["p", "b"]),
];

/// Get the phonetic consonants for a single digit (0–9).
pub fn phonetics_for_digit(d: u8) -> &'static [&'static str] {
    let group = if d >= 5 { (d - 5) as usize } else { d as usize };
    let pair = &PHONETIC_PAIRS[group.min(4)];
    if d >= 5 { pair.3 } else { pair.2 }
}

/// Fold phonetics for a 2-digit pair where both digits are the same number (or same folding group).
/// Returns (first_phonetics, fold_to_phonetics).
pub fn fold_phonetics(digit: u8) -> (&'static [&'static str], &'static [&'static str]) {
    let group = if digit >= 5 { (digit - 5) as usize } else { digit as usize };
    let pair = &PHONETIC_PAIRS[group.min(4)];
    if digit >= 5 {
        (pair.3, pair.2) // upper digit: paired→primary
    } else {
        (pair.2, pair.3) // lower digit: primary→paired
    }
}

/// Encode a 2-digit index pair into phonetic consonant strings.
/// Folding rule: if both digits belong to the same folding pair (0/5, 1/6, 2/7, 3/8, 4/9),
/// the second digit falls back to the paired group's phonetic, delimited by "o".
pub fn encode_2digit(d1: u8, d2: u8) -> Vec<String> {
    if d1 % 5 == d2 % 5 {
        // Same folding group → apply folding rule
        let (first, fold) = fold_phonetics(d1);
        let mut result = Vec::new();
        result.extend(first.iter().map(|s| s.to_string()));
        result.push("o".to_string());
        result.extend(fold.iter().map(|s| s.to_string()));
        result
    } else {
        let mut result = Vec::new();
        let ph1 = phonetics_for_digit(d1);
        let ph2 = phonetics_for_digit(d2);
        result.push(ph1[0].to_string());
        result.push(ph2[0].to_string());
        result
    }
}

/// Encode a 6-digit index string (e.g. "000000" or "00-00-00") into its phonetic representation.
pub fn encode_index(index: &str) -> Vec<String> {
    let digits: Vec<u8> = index
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect();

    if digits.len() < 2 {
        return digits.iter().map(|d| phonetics_for_digit(*d)[0].to_string()).collect();
    }

    let pairs = digits.chunks(2);
    let mut result = Vec::new();
    for (i, pair) in pairs.enumerate() {
        if i > 0 {
            result.push("-".to_string());
        }
        if pair.len() == 2 {
            result.extend(encode_2digit(pair[0], pair[1]));
        } else {
            result.push(phonetics_for_digit(pair[0])[0].to_string());
        }
    }
    result
}

pub fn phonetic_word(index: &str) -> String {
    encode_index(index).join("")
}

// ── 4-Quadrant Letter Mapping ───────────────────────────────
//
// 00-99 is divided into 4 sets of 25 values each:
//   Set 1 (00-24): a, b, ..., y     (ascending lowercase, z omitted)
//   Set 3 (25-49): -y, -x, ..., -a  (descending lowercase with hyphen prefix)
//   Set 4 (50-74): -A, -B, ..., -Y  (ascending uppercase with hyphen prefix)
//   Set 2 (75-99): Y, X, ..., A     (descending uppercase)

/// Convert a 2-digit number (00–99) to its letter representation based on the 4-quadrant scheme.
pub fn digit_pair_to_letters(d2: &str) -> String {
    let num: u32 = d2.parse().unwrap_or(0);
    match num {
        0..=24 => {
            // Set 1: a=00, b=01, ..., y=24
            let c = (b'a' + num as u8) as char;
            c.to_string()
        }
        25..=49 => {
            // Set 3: -y=25, -x=26, ..., -a=49
            let offset = num - 25;
            let c = (b'y' - offset as u8) as char;
            format!("-{}", c)
        }
        50..=74 => {
            // Set 4: -A=50, -B=51, ..., -Y=74
            let offset = num - 50;
            let c = (b'A' + offset as u8) as char;
            format!("-{}", c)
        }
        75..=99 => {
            // Set 2: Y=75, X=76, ..., A=99
            let offset = num - 75;
            let c = (b'Y' - offset as u8) as char;
            c.to_string()
        }
        _ => d2.to_string(),
    }
}

/// Convert a letter-based value to phonetic consonants.
/// Interprets each letter as a 2-digit number and applies the 2-digit encoding.
pub fn letters_to_phonetic(letters: &str) -> String {
    let mut result = String::new();
    for c in letters.chars() {
        if c == '-' {
            continue; // skip hyphens (they are inverted markers)
        }
        if c.is_ascii_alphabetic() {
            let idx = (c.to_ascii_uppercase() as u8 - b'A') as u32;
            let d1 = ((idx / 10) % 10) as u8;
            let d2 = (idx % 10) as u8;
            let ph = encode_2digit(d1, d2);
            for p in &ph {
                if p != "-" && p != "o" {
                    result.push_str(p);
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Parse a 2-digit letter value back into its numeric 00-99 equivalent.
pub fn letters_to_digit_pair(letters: &str) -> String {
    let clean = letters.trim_start_matches('-');
    if clean.is_empty() {
        return "00".to_string();
    }
    let is_inverted = letters.starts_with('-');
    let is_upper = clean.chars().next().map_or(false, |c| c.is_ascii_uppercase());

    if clean.len() == 1 {
        let c = clean.chars().next().unwrap();
        if is_inverted && !is_upper {
            // Set 3: -a..=-y → 49..=25
            let offset = b'y' - c.to_ascii_lowercase() as u8;
            format!("{:02}", 25 + offset as u32)
        } else if is_inverted && is_upper {
            // Set 4: -A..=-Y → 50..=74
            let offset = c as u8 - b'A';
            format!("{:02}", 50 + offset as u32)
        } else if !is_inverted && is_upper {
            // Set 2: A..=Y → 99..=75
            let offset = b'Y' - c as u8;
            format!("{:02}", 75 + offset as u32)
        } else {
            // Set 1: a..=y → 00..=24
            let offset = c as u8 - b'a';
            format!("{:02}", offset as u32)
        }
    } else {
        "00".to_string()
    }
}

// ── Registry Types ──────────────────────────────────────────

/// A 6-digit registry index, structured as [Registry][Index][Closing].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RegistryIndex {
    /// First 2 digits: registry identifier.
    pub registry: String,
    /// Second 2 digits: the index within the registry.
    pub idx: String,
    /// Third 2 digits: closing index / placeholder.
    pub closing: String,
}

impl RegistryIndex {
    pub fn from_6digit(s: &str) -> Self {
        let clean: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
        let padded = format!("{:0<6}", clean);
        Self {
            registry: padded[0..2].to_string(),
            idx: padded[2..4].to_string(),
            closing: padded[4..6].to_string(),
        }
    }

    /// Convert to letter-based representation using the 4-quadrant scheme.
    pub fn to_letters(&self) -> String {
        format!(
            "{}-{}-{}",
            digit_pair_to_letters(&self.registry),
            digit_pair_to_letters(&self.idx),
            digit_pair_to_letters(&self.closing),
        )
    }

    /// Convert to number-based representation.
    pub fn to_numbers(&self) -> String {
        format!("{}-{}-{}", self.registry, self.idx, self.closing)
    }

    /// Alternating display: start with letters or numbers, alternating per group.
    pub fn to_alternating(&self, start_with_letters: bool) -> String {
        let groups = [&self.registry, &self.idx, &self.closing];
        let parts: Vec<String> = groups.iter().enumerate().map(|(i, g)| {
            let should_be_letters = if start_with_letters {
                i % 2 == 0
            } else {
                i % 2 == 1
            };
            if should_be_letters {
                digit_pair_to_letters(g)
            } else {
                g.to_string()
            }
        }).collect();
        parts.join("-")
    }

    /// Phonetic display: convert letter values to their phonetic equivalents.
    /// Inverted indices (between 2nd and 3rd groups) are capitalized.
    pub fn to_phonetic_display(&self, inverted_indices: &[String]) -> String {
        let mut parts = Vec::new();
        let reg_letters = digit_pair_to_letters(&self.registry);
        parts.push(letters_to_phonetic(&reg_letters));

        let idx_letters = digit_pair_to_letters(&self.idx);
        parts.push(letters_to_phonetic(&idx_letters));

        for inv in inverted_indices {
            let ph = letters_to_phonetic(inv);
            parts.push(ph.to_uppercase());
        }

        let close_letters = digit_pair_to_letters(&self.closing);
        parts.push(letters_to_phonetic(&close_letters));

        parts.join("-")
    }
}

// ── Indexed PAO Entry ───────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndexedPAO {
    /// 6-digit index (with or without separators).
    pub index: String,
    pub person: String,
    pub action: String,
    pub object: String,
}

impl IndexedPAO {
    pub fn new(index: &str, person: &str, action: &str, object: &str) -> Self {
        Self {
            index: index.to_string(),
            person: person.to_string(),
            action: action.to_string(),
            object: object.to_string(),
        }
    }
}

// ── History Entry ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HistoryEntry {
    pub timestamp: String,
    pub index: String,
    pub entry_type: String,
    pub content: String,
    pub context: Option<String>,
}

// ── Registry Metadata ───────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegistryMeta {
    pub prefix: String,
    pub description: String,
    pub quadrant: u8, // 1=00-24, 2=75-99, 3=25-49, 4=50-74
    pub created: String,
}

// ── Indexing System ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndexingSystem {
    pub entries: HashMap<String, IndexedPAO>,
    /// Routing table: empty_slot -> routed_to index.
    pub routing: HashMap<String, String>,
    /// History registry.
    pub history: Vec<HistoryEntry>,
    /// Reserved indices (locked from reuse).
    pub reservations: HashSet<String>,
    /// Registry metadata: maps registry prefix to metadata.
    pub registries: HashMap<String, RegistryMeta>,
    /// Inverted indices: maps a 2-digit pair to its inverted counterpart.
    pub inverted: HashMap<String, String>,
}

impl Default for IndexingSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl IndexingSystem {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            routing: HashMap::new(),
            history: Vec::new(),
            reservations: HashSet::new(),
            registries: HashMap::new(),
            inverted: HashMap::new(),
        }
    }

    // ── Entry management ──

    pub fn insert(&mut self, index: &str, person: &str, action: &str, object: &str) {
        let entry = IndexedPAO::new(index, person, action, object);
        self.entries.insert(index.to_string(), entry);
    }

    pub fn insert_entry(&mut self, entry: IndexedPAO) {
        let idx = entry.index.clone();
        self.entries.insert(idx, entry);
    }

    pub fn get(&self, index: &str) -> Option<&IndexedPAO> {
        self.entries.get(index)
    }

    pub fn is_empty(&self, index: &str) -> bool {
        !self.entries.contains_key(index)
    }

    pub fn remove(&mut self, index: &str) -> Option<IndexedPAO> {
        self.entries.remove(index)
    }

    pub fn all_entries(&self) -> Vec<&IndexedPAO> {
        let mut vec: Vec<&IndexedPAO> = self.entries.values().collect();
        vec.sort_by(|a, b| a.index.cmp(&b.index));
        vec
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    // ── Routing / Branching ──

    /// Add a route from an empty slot to a target index.
    pub fn add_route(&mut self, empty_slot: &str, routed_to: &str) {
        self.routing.insert(empty_slot.to_string(), routed_to.to_string());
    }

    pub fn resolve_route(&self, empty_slot: &str) -> Option<&String> {
        self.routing.get(empty_slot)
    }

    pub fn get_or_route(&self, index: &str) -> Option<&IndexedPAO> {
        self.get(index).or_else(|| {
            self.routing.get(index).and_then(|routed| self.get(routed))
        })
    }

    /// Resolve an index through branching chains.
    pub fn resolve_branch(&self, index: &str) -> Option<&IndexedPAO> {
        if let Some(entry) = self.get(index) {
            return Some(entry);
        }
        if let Some(routed) = self.resolve_route(index) {
            return self.get(routed);
        }
        let ri = RegistryIndex::from_6digit(index);
        let branch_key = format!("{}:branch", ri.registry);
        if let Some(branch_to) = self.routing.get(&branch_key) {
            let branch_idx = format!("{}-{}-{}", branch_to, ri.idx, ri.closing);
            return self.get(&branch_idx);
        }
        None
    }

    // ── Inverted Indices ──

    pub fn add_inverted(&mut self, from: &str, to: &str) {
        self.inverted.insert(from.to_string(), to.to_string());
    }

    pub fn get_inverted(&self, from: &str) -> Option<&String> {
        self.inverted.get(from)
    }

    pub fn all_inverted(&self) -> Vec<(&String, &String)> {
        let mut v: Vec<_> = self.inverted.iter().collect();
        v.sort_by(|a, b| a.0.cmp(b.0));
        v
    }

    // ── Reservation & Intellisense ──

    /// Reserve an index (lock it from reuse).
    pub fn reserve(&mut self, index: &str) -> bool {
        if self.reservations.contains(index) {
            return false;
        }
        self.reservations.insert(index.to_string());
        true
    }

    pub fn unreserve(&mut self, index: &str) {
        self.reservations.remove(index);
    }

    pub fn is_reserved(&self, index: &str) -> bool {
        self.reservations.contains(index)
    }

    /// Check for conflicts: returns a list of conflict descriptions.
    pub fn check_conflicts(&self, index: &str) -> Vec<String> {
        let clean: String = index.chars().filter(|c| c.is_ascii_digit()).collect();
        let mut conflicts = Vec::new();

        if self.entries.contains_key(index) || self.reservations.contains(index) {
            conflicts.push(format!("Direct conflict: {index} already taken"));
        }

        if clean.len() == 6 {
            let ri = RegistryIndex::from_6digit(index);

            // Check symmetric conflicts (mirrored indices)
            let mirrored = format!("{}{}{}", reverse_2digit(&ri.registry), ri.idx, ri.closing);
            if self.entries.contains_key(&mirrored) || self.reservations.contains(&mirrored) {
                conflicts.push(format!("Symmetry conflict with mirrored index {mirrored}"));
            }

            // Check if registry prefix has a branch that would create a conflict
            let branch_key = format!("{}:branch", ri.registry);
            if let Some(branch_to) = self.routing.get(&branch_key) {
                let branched_idx = format!("{}-{}-{}", branch_to, ri.idx, ri.closing);
                if self.entries.contains_key(&branched_idx) || self.reservations.contains(&branched_idx) {
                    conflicts.push(format!("Branch conflict with {branched_idx} (via registry {})", ri.registry));
                }
            }

            // Check inverted index conflicts
            let inv_key = format!("{}-{}", ri.registry, ri.idx);
            if self.inverted.contains_key(&inv_key) {
                conflicts.push(format!("Inverted index exists for {inv_key}"));
            }
        }

        conflicts
    }

    /// Forward-looking intellisense: given a partial index, suggest safe alternative indices.
    pub fn suggest_safe_indices(&self, partial: &str) -> Vec<String> {
        let clean: String = partial.chars().filter(|c| c.is_ascii_digit()).collect();
        let mut suggestions = Vec::new();

        let base = format!("{:0<6}", clean);
        if self.entries.contains_key(&base) || self.reservations.contains(&base) {
            let ri = RegistryIndex::from_6digit(&base);
            let close_num: u32 = ri.closing.parse().unwrap_or(0);
            for offset in 1..=10 {
                let new_close_num = (close_num + offset) % 100;
                let new_close = format!("{:02}", new_close_num);
                let candidate = format!("{}-{}-{}", ri.registry, ri.idx, new_close);
                if !self.entries.contains_key(&candidate) && !self.reservations.contains(&candidate) {
                    suggestions.push(candidate);
                    if suggestions.len() >= 3 {
                        break;
                    }
                }
            }
        }

        suggestions
    }

    // ── Registry Management ──

    /// Register a new registry prefix with its quadrant.
    pub fn register_registry(&mut self, prefix: &str, description: &str) {
        let num: u32 = prefix.parse().unwrap_or(0);
        let quadrant = match num {
            0..=24 => 1,
            75..=99 => 2,
            25..=49 => 3,
            50..=74 => 4,
            _ => 0,
        };
        self.registries.insert(prefix.to_string(), RegistryMeta {
            prefix: prefix.to_string(),
            description: description.to_string(),
            quadrant,
            created: chrono::Utc::now().to_rfc3339(),
        });
    }

    pub fn get_registry(&self, prefix: &str) -> Option<&RegistryMeta> {
        self.registries.get(prefix)
    }

    pub fn all_registries(&self) -> Vec<&RegistryMeta> {
        let mut v: Vec<_> = self.registries.values().collect();
        v.sort_by(|a, b| a.prefix.cmp(&b.prefix));
        v
    }

    // ── History ──

    pub fn add_history(&mut self, entry: HistoryEntry) {
        self.history.push(entry);
    }

    pub fn history(&self) -> Vec<&HistoryEntry> {
        let mut h: Vec<&HistoryEntry> = self.history.iter().collect();
        h.reverse();
        h
    }

    pub fn history_for_index(&self, index: &str) -> Vec<&HistoryEntry> {
        self.history.iter()
            .filter(|e| e.index == index)
            .collect()
    }

    pub fn history_by_type(&self, entry_type: &str) -> Vec<&HistoryEntry> {
        self.history.iter()
            .filter(|e| e.entry_type == entry_type)
            .collect()
    }

    pub fn capture_history(
        &mut self,
        index: &str,
        entry_type: &str,
        content: &str,
        context: Option<String>,
    ) {
        let entry = HistoryEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            index: index.to_string(),
            entry_type: entry_type.to_string(),
            content: content.to_string(),
            context,
        };
        self.add_history(entry);
    }

    // ── Persistence ──

    pub fn default_data_dir() -> String {
        "data/values".to_string()
    }

    pub fn default_store_path() -> String {
        format!("{}/indexing_store.json", Self::default_data_dir())
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        if let Some(parent) = Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)
    }

    pub fn load_from_file(path: &str) -> std::io::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let sys: IndexingSystem = serde_json::from_str(&json)?;
        Ok(sys)
    }

    pub fn save(&self) -> std::io::Result<()> {
        self.save_to_file(&Self::default_store_path())
    }

    pub fn load() -> Self {
        let path = Self::default_store_path();
        if Path::new(&path).exists() {
            Self::load_from_file(&path).unwrap_or_default()
        } else {
            Self::new()
        }
    }

    pub fn load_or_default() -> Self {
        let mut sys = Self::load();
        if sys.routing.is_empty() {
            sys.routing = default_routing();
        }
        if sys.registries.is_empty() {
            // Register default registries for each quadrant
            sys.register_registry("00", "Set 1: forward lowercase (a→y)");
            sys.register_registry("25", "Set 3: inverted lowercase (-y→-a)");
            sys.register_registry("50", "Set 4: inverted uppercase (-A→-Y)");
            sys.register_registry("75", "Set 2: reverse uppercase (Y→A)");
        }
        sys
    }

    // ── Export / Import ──

    /// Export the current indexing system to a JSON string.
    pub fn export_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Import and replace the current indexing system from a JSON string.
    pub fn import_from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Export just the entries (PAO data) to a simplified JSON format.
    pub fn export_entries_to_json(&self) -> Result<String, serde_json::Error> {
        let pairs: Vec<_> = self.entries.values()
            .map(|e| {
                (
                    e.index.clone(),
                    format!("{} | {} | {}", e.person, e.action, e.object)
                )
            })
            .collect();
        serde_json::to_string_pretty(&pairs)
    }
}

// ── Helper Functions ────────────────────────────────────────

fn reverse_2digit(d: &str) -> String {
    let chars: Vec<char> = d.chars().collect();
    if chars.len() == 2 {
        format!("{}{}", chars[1], chars[0])
    } else {
        d.to_string()
    }
}

/// Decode a phonetic string back to digits.
pub fn decode_phonetics(phonetics: &[String]) -> String {
    let mut result = String::new();
    for ph in phonetics {
        let lower = ph.to_lowercase();
        if lower == "o" {
            continue;
        }
        let mut found = false;
        for (_, _, primary, paired) in &PHONETIC_PAIRS {
            if primary.iter().any(|s| *s == lower) {
                // Find which digit in the group
                result.push_str(primary[0]); // Use group digit
                found = true;
                break;
            }
            if paired.iter().any(|s| *s == lower) {
                result.push_str(paired[0]);
                found = true;
                break;
            }
        }
        if !found {
            result.push_str(ph);
        }
    }
    result
}

/// Default routing setup for the 4-quadrant inverted index system.
///
/// The four routing sets:
///   Set 1: 00,a → 24,y    (00–24, forward lowercase a→y)
///   Set 3: 25,-y → 49,-a  (25–49, inverted descending -y→-a)
///   Set 4: 50,-A → 74,-Y  (50–74, inverted ascending -A→-Y)
///   Set 2: 75,Y → 99,A    (75–99, reverse uppercase Y→A)
pub fn default_routing() -> HashMap<String, String> {
    let mut routes = HashMap::new();
    // Branch routing: each quadrant's registry can branch to another
    routes.insert("00:branch".to_string(), "24".to_string());
    routes.insert("25:branch".to_string(), "49".to_string());
    routes.insert("50:branch".to_string(), "74".to_string());
    routes.insert("75:branch".to_string(), "99".to_string());
    // Inverted index routing (between 2nd and 3rd groups)
    routes.insert("00-inv".to_string(), "-y".to_string());
    routes.insert("25-inv".to_string(), "-a".to_string());
    routes.insert("50-inv".to_string(), "-Y".to_string());
    routes.insert("75-inv".to_string(), "A".to_string());
    routes
}

/// Generate a display of the given index in all 5 modes.
pub fn display_all_modes(index: &str, inverted: &[String]) -> Vec<(String, String)> {
    let ri = RegistryIndex::from_6digit(index);
    vec![
        ("Letters".to_string(), ri.to_letters()),
        ("Numbers".to_string(), ri.to_numbers()),
        ("Alternating (L-N-L)".to_string(), ri.to_alternating(true)),
        ("Alternating (N-L-N)".to_string(), ri.to_alternating(false)),
        ("Phonetic".to_string(), ri.to_phonetic_display(inverted)),
    ]
}

/// Get the quadrant label for a 2-digit number.
pub fn quadrant_label(num: u32) -> &'static str {
    match num {
        0..=24 => "Set 1 (00-24): a→y forward lowercase",
        25..=49 => "Set 3 (25-49): -y→-a inverted descending",
        50..=74 => "Set 4 (50-74): -A→-Y inverted ascending",
        75..=99 => "Set 2 (75-99): Y→A reverse uppercase",
        _ => "Unknown",
    }
}

// ── Tests ───────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_import_roundtrip() {
        let mut sys = IndexingSystem::new();
        sys.insert("001001", "Alice", "runs", "ball");
        sys.insert("002002", "Bob", "jumps", "car");
        sys.reserve("003003");
        sys.capture_history("001001", "note", "Created entry", None);

        let json = sys.export_to_json().expect("export should work");
        let imported = IndexingSystem::import_from_json(&json).expect("import should work");

        assert_eq!(imported.len(), 2);
        assert_eq!(imported.get("001001").unwrap().person, "Alice");
        assert_eq!(imported.get("002002").unwrap().action, "jumps");
        assert!(imported.is_reserved("003003"));
        assert_eq!(imported.history().len(), 1);
    }

    #[test]
    fn test_export_entries() {
        let mut sys = IndexingSystem::new();
        sys.insert("001001", "Alice", "runs", "ball");
        sys.insert("002002", "Bob", "jumps", "car");
        let json = sys.export_entries_to_json().expect("export entries should work");
        assert!(json.contains("001001"));
        assert!(json.contains("Alice"));
        assert!(json.contains("Bob"));
    }
}