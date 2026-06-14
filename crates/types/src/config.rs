#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpenserConfig {
    pub vcs: String,
    pub producer: String,
}

impl Default for SpenserConfig {
    fn default() -> Self {
        Self {
            vcs: "git".to_string(),
            producer: "builtin:git-standard".to_string(),
        }
    }
}