use std::path::PathBuf;

/// The producer's output. Fixed outer shape, open inner enrichments.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SchemaDiff {
    pub schema_name: String,
    pub files: Vec<FileDiff>,
}

/// One file's worth of changes.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileDiff {
    pub old_path: Option<PathBuf>,
    pub new_path: Option<PathBuf>,
    pub status: FileStatus,
    pub regions: Vec<Region>,
    pub enrichments: Enrichments,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum FileStatus {
    Added,
    Deleted,
    Modified,
    Renamed,
}

/// One contiguous changed region within a file.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Region {
    pub range:       HunkRange,
    pub lines:       Vec<DiffLine>,
    pub enrichments: Enrichments,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegionRef {
    pub path:  PathBuf,
    pub range: HunkRange,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HunkRange {
    pub old_start: u32,
    pub old_lines: u32,
    pub new_start: u32,
    pub new_lines: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiffLine {
    pub kind: LineKind,
    pub content: String,
    pub old_no: Option<u32>,
    pub new_no: Option<u32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum LineKind {
    Added,
    Removed,
    Context,
}

/// The open slot for user-defined enrichments.
/// Types crate has no opinion about what goes in here.
/// Capability readers in core extract typed values by key.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Enrichments {
    #[serde(flatten)]
    pub fields: serde_json::Value,
}

impl Enrichments {
    pub fn new() -> Self {
        Self {
            fields: serde_json::Value::Object(serde_json::Map::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.fields.get(key)
    }

    pub fn insert(&mut self, key: &str, value: serde_json::Value) {
        match self.fields {
            serde_json::Value::Object(ref mut map) => {
                map.insert(key.to_string(), value);
            }
            _ => {
                let mut map = serde_json::Map::new();
                map.insert(key.to_string(), value);
                self.fields = serde_json::Value::Object(map);
            }
        }
    }
}