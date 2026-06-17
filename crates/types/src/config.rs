#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpenserConfig {
    pub vcs: String,
    pub producer: String,
    pub slicer: String,
    pub slicer_config: serde_json::Value,
}

impl Default for SpenserConfig {
    fn default() -> Self {
        Self {
            vcs: "git".to_string(),
            producer: "builtin:git-standard".to_string(),
            slicer: "builtin:region".to_string(),
            slicer_config: serde_json::Value::Null,
        }
    }
}