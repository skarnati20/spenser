use std::fmt;

use uuid::Uuid;

// minted, permanent — never derived from content
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ChangeId(String);
impl ChangeId {
    pub fn new() -> Self { Self(Uuid::new_v4().to_string()) }
    pub fn as_str(&self) -> &str { &self.0 }
}

impl fmt::Display for ChangeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct RoundId(String);
impl RoundId { pub fn new() -> Self { Self(Uuid::new_v4().to_string()) } }

impl fmt::Display for RoundId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CardId(String);
impl CardId {
    pub fn new() -> Self { Self(Uuid::new_v4().to_string()) }
    pub fn as_str(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct UserId(String);

// a recorded fact, not an identity — the newtype prevents accidental use as one
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CommitHash(pub String);

impl CommitHash {
    /// The conventional 7-character abbreviated hash.
    pub fn short(&self) -> &str {
        let len = self.0.len().min(7);
        &self.0[..len]
    }
}