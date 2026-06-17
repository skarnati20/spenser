use std::env;

use anyhow::Result;
use core::dir::{find_spenser_dir, load_session};

pub fn execute(session_id: String) -> Result<()> {
    let current_dir = env::current_dir()?;
    let spenser_dir = find_spenser_dir(&current_dir)
        .ok_or_else(|| anyhow::anyhow!("no .spenser found, run `spenser init` first"))?;

    load_session(&spenser_dir, &session_id)?;
    println!("switched to session {}", session_id);
    Ok(())
}
