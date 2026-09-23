//! The frozen benchmark manifest (vob-1.1) — consumed artifact.
//!
//! Ported from hyprstream-bench's manifest module (subset): this app only
//! READS the manifest to run the family/item contamination firewall. The
//! manifest itself is produced by the platform's benchmark (P0.6) and
//! pinned by digest.

use serde::{Deserialize, Serialize};

pub const RELEASE: &str = "vob-1.1";

pub struct ManifestItem {
    /// Item id.
    pub id: String,
    /// blake3 hex digest of the item's canonical bytes.
    pub blake3: String,
    /// Family id.
    pub family: String,
    /// Stratum wire form (`clean` / `nearmiss` / `perm-N`).
    pub stratum: String,
    /// Permutation group id.
    pub group: String,
    /// Question kind (`noul` / `choice` / `score`).
    pub kind: String,
    /// Correct label index in canonical label order.
    pub truth: usize,
}

/// Family designation row — the family-level holdout firewall.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FamilyRow {
    /// Family id.
    pub id: String,
    /// `open` (train-usable) or `gate` (never synthesized; zero-shot gate).
    pub designation: String,
}

/// The frozen manifest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    /// Release id ([`RELEASE`]).
    /// Release id ([`RELEASE`]).
    pub release: String,
    /// Harness crate + version that generated this manifest.
    pub harness: String,
    /// Pinned generation config.
    pub config: ManifestConfig,
    /// Family designations (the firewall).
    pub families: Vec<FamilyRow>,
    /// License pointers: harness Apache-2.0, items CC-BY-4.0.
    pub license: ManifestLicense,
    /// Path of the pre-committed disclosure text + its blake3 digest.
    pub disclosure: ManifestDisclosure,
    /// Total item count.
    pub item_count: usize,
    /// Per-item rows, in generation order.
    pub items: Vec<ManifestItem>,
}

/// Pinned generation config inside the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestConfig {
    /// Base items per (family, base stratum, kind).
    pub seeds_per_stratum: u32,
    /// Seed stream base (hex).
    pub seed_base: String,
}

/// License pointers inside the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestLicense {
    /// Harness license (Apache-2.0).
    pub harness: String,
    /// Generated-items license (CC-BY-4.0).
    pub items: String,
}

/// Disclosure pointer inside the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestDisclosure {
    /// Repo-relative path of the disclosure text.
    pub path: String,
    /// blake3 hex digest of the disclosure file at freeze time.
    pub blake3: String,
}

/// Errors from manifest verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyError {
    /// Manifest release id mismatch.
    ReleaseMismatch { expected: String, found: String },
    /// Item count mismatch.
    CountMismatch { expected: usize, found: usize },
    /// Item at `index` diverged (first divergence reported).
    ItemMismatch { index: usize, id: String },
    /// A family designation changed (firewall violation).
    DesignationChanged { family: String },
}


impl Manifest {
    /// Parse a manifest from JSON.
    pub fn from_json(text: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(text)
    }
}

/// blake3 hex digest of a file's bytes (used to pin DISCLOSURE.md).
pub fn blake3_hex(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(64);
    for byte in blake3::hash(bytes).as_bytes() {
        let _ = write!(hex, "{byte:02x}");
    }
