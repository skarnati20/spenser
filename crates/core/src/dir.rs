use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use types::{Session, SpenserConfig};

pub const SPENSER_DIR: &str = ".spenser";

pub fn find_spencer_dir(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        let candidate = current.join(SPENSER_DIR);
        if candidate.exists() && candidate.is_dir() {
            return Some(candidate);
        }
        if !current.pop() {
            // hit the root, nowhere left to go
            return None;
        }
    }
}

pub fn write_default_config(spenser_dir: &Path) -> Result<()> {
    let config = SpenserConfig::default();
    let json = serde_json::to_string_pretty(&config)?;
    fs::write(spenser_dir.join("config.json"), json)?;
    Ok(())
}

pub fn read_config(spenser_dir: &Path) -> Result<SpenserConfig> {
    let content = fs::read_to_string(spenser_dir.join("config.json"))?;
    let config: SpenserConfig = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn create_session(spenser_dir: &Path, description: String) -> Result<Session> {
    let session = Session::new(description);
    let json = serde_json::to_string_pretty(&session)?;
    fs::write(spenser_dir.join("session.json"), json)?;
    Ok(session)
}

pub fn read_session(spenser_dir: &Path) -> Option<Session> {
    let path = spenser_dir.join("session.json");
    if !path.exists() {
        return None;
    }
    let content = fs::read_to_string(&path).ok()?;
    let session: Session = serde_json::from_str(&content).ok()?;
    Some(session)
}

pub fn write_session(spenser_dir: &Path, session: &Session) -> Result<()> {
    let json = serde_json::to_string_pretty(session)?;
    fs::write(spenser_dir.join("session.json"), json)?;
    Ok(())
}

pub fn archive_session(spenser_dir: &Path) -> Result<()> {
    let session = read_session(spenser_dir)
        .ok_or_else(|| anyhow::anyhow!("no active session in {}", spenser_dir.display()))?;
    let archive_dir = spenser_dir.join("archive");
    fs::create_dir_all(&archive_dir)?;

    let filename = format!("{}.json", session.id);
    fs::rename(
        spenser_dir.join("session.json"),
        archive_dir.join(filename),
    )?;

    // remove stale anchor
    let anchor = spenser_dir.join("anchor");
    if anchor.exists() {
        fs::remove_file(anchor)?;
    }

    Ok(())
}