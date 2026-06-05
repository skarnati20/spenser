use crate::diff::RegionRef;
use crate::ids::{CardId, UserId};

/// The primary unit of review. Owns its current content and conversation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Card {
    pub id: CardId,
    pub content: CardContent,
    pub thread: Vec<Response>,
}

impl Card {
    pub fn new(content: CardContent) -> Self {
        Self {
            id: CardId::new(),
            content,
            thread: Vec::new(),
        }
    }
}

/// What a reviewer sees for this card right now.
/// Replaced on re-slice; old state is recoverable from the round's diff.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CardContent {
    pub body: String,
    pub region_ref: RegionRef,
}

/// One response in a card's conversation thread.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Response {
    pub author: UserId,
    pub region_ref: RegionRef,
    pub text: String,
}