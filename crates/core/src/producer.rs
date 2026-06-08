use anyhow::Result;
use types::{CommitHash, SchemaDiff};

pub trait Producer {
    fn name(&self) -> &str;
    fn produce(&self, base: &CommitHash, head: &CommitHash) -> Result<SchemaDiff>;
}