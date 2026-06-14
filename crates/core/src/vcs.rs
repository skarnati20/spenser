use anyhow::Result;
use types::CommitHash;

pub trait Vcs {
    fn resolve(&self, refspec: &str) -> Result<CommitHash>;
}