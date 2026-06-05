use crate::diff::SchemaDiff;
use crate::ids::{CardId, CommitHash, RoundId};

/// An immutable snapshot of one review cycle.
/// Records what was diffed and which cards were active.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Round {
    pub id: RoundId,
    pub base_hash: CommitHash,
    pub head_hash: CommitHash,
    pub diff: SchemaDiff,
    pub card_refs: Vec<CardId>,
}

impl Round {
    pub fn new(
        base_hash: CommitHash,
        head_hash: CommitHash,
        diff: SchemaDiff,
    ) -> Self {
        Self {
            id: RoundId::new(),
            base_hash,
            head_hash,
            diff,
            card_refs: Vec::new(),
        }
    }
}