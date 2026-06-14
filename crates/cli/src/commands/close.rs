use anyhow::{bail, Result};
use core::dir::{find_spenser_dir, read_session, write_session};
use std::env;

pub fn execute() -> Result<()> {
    let current_dir = env::current_dir()?;
    let spenser_dir = find_spenser_dir(&current_dir)
        .ok_or_else(|| anyhow::anyhow!("no .spenser directory found"))?;

    let mut session = read_session(&spenser_dir)
        .ok_or_else(|| anyhow::anyhow!("no active session"))?;

    if !session.is_open() {
        bail!("session {} is already closed", session.id);
    }

    session.close();
    write_session(&spenser_dir, &session)?;
    println!("closed session {}", session.id);
    Ok(())
}