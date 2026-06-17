use anyhow::{Result, bail};
use core::Vcs;
use git::GitRepo;
use types::SpenserConfig;

pub fn open_vcs(config: &SpenserConfig, path: &str) -> Result<Box<dyn Vcs>> {
    match config.vcs.as_str() {
        "git" => Ok(Box::new(GitRepo::open(path)?)),
        other => bail!("unsupported vcs: {other}"),
    }
}
