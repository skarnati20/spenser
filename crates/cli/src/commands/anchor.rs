use anyhow::{bail, Result};
use std::env;
use std::fs;

use core::dir::{find_spenser_dir, read_config, read_session};
use registry::open_vcs;

pub fn execute(commit: &str) -> Result<()> {
    let current_dir = env::current_dir()?;
    let spenser_dir = find_spenser_dir(&current_dir)
        .ok_or_else(|| anyhow::anyhow!("no .spenser found, run `spenser init` first"))?;

    let session = read_session(&spenser_dir)
        .ok_or_else(|| anyhow::anyhow!("no active session, run `spenser init` first"))?;
    if !session.is_open() {
        bail!("session is closed");
    }

    let config = read_config(&spenser_dir)?;
    let vcs = open_vcs(&config, ".")?;
    let hash = vcs.resolve(commit)?;

    fs::write(spenser_dir.join("anchor"), hash.0.as_str())?;
    println!("anchored at {}", hash.short());
    Ok(())
}