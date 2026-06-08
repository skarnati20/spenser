use anyhow::Result;
use core::Producer;
use types::{CommitHash, SchemaDiff};

use crate::repo::GitRepo;

pub struct GitStandardProducer {
    pub path: String,
}

impl GitStandardProducer {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

impl Producer for GitStandardProducer {
    fn name(&self) -> &str {
        "git_standard"
    }

    fn produce(&self, base: &CommitHash, head: &CommitHash) -> Result<SchemaDiff> {
        let repo = GitRepo::open(&self.path)?;
        let files = repo.diff(base, head)?;
        Ok(SchemaDiff {
            schema_name: self.name().to_string(),
            files,
        })
    }
}
