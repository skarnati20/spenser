#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpenserConfig {
    pub producer: String,
}

impl Default for SpenserConfig {
    fn default() -> Self {
        Self {
            producer: "builtin:git-standard".to_string(),
        }
    }
}