use anyhow::Result;
use std::env;
use std::fs;

use core::dir::{find_spenser_dir, read_session};
use types::Session;

pub fn execute() -> Result<()> {
    let cwd = env::current_dir()?;
    let dir = find_spenser_dir(&cwd)
        .ok_or_else(|| anyhow::anyhow!("no .spenser found"))?;

    // current session
    let current = read_session(&dir)
        .ok_or_else(|| anyhow::anyhow!("no active session, run `spenser init` first"))?;
    println!("* {} ({})  \"{}\"  [{} round(s)]",
        current.id,
        if current.is_open() { "Open" } else { "Closed" },
        current.description,
        current.rounds.len());

    // archived sessions
    let archive_dir = dir.join("archive");
    if archive_dir.exists() {
        for entry in fs::read_dir(&archive_dir)? {
            let path = entry?.path();
            if path.extension().map_or(false, |e| e == "json") {
                let content = fs::read_to_string(&path)?;
                if let Ok(session) = serde_json::from_str::<Session>(&content) {
                    println!("  {} ({})  \"{}\"  [{} round(s)]",
                        session.id,
                        if session.is_open() { "Open" } else { "Closed" },
                        session.description,
                        session.rounds.len());
                }
            }
        }
    }

    Ok(())
}