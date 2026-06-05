use uuid::Uuid;

// minted, permanent — never derived from content
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ChangeId(String);
impl ChangeId {
    pub fn new() -> Self { Self(Uuid::new_v4().to_string()) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct RoundId(String);
impl RoundId { pub fn new() -> Self { Self(Uuid::new_v4().to_string()) } }

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CardId(String);
impl CardId { pub fn new() -> Self { Self(Uuid::new_v4().to_string()) } }

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct UserId(String);

// a recorded fact, not an identity — the newtype prevents accidental use as one
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CommitHash(pub String);