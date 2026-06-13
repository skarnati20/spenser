use std::env;
use std::fs;

use anyhow::{bail, Result};
use core::dir::{
    SPENSER_DIR,
    archive_session,
    create_session,
    find_spencer_dir,
    read_session,
    write_default_config,
};

pub fn execute(description: String) -> Result<()> {
    let current_dir = env::current_dir()?;
    let spenser_dir = find_spencer_dir(&current_dir);

    let spenser_dir = match spenser_dir {
        Some(dir) => dir,
        None => {
            let new_dir = current_dir.join(SPENSER_DIR);
            fs::create_dir(&new_dir)?;
            write_default_config(&new_dir)?;
            let session = create_session(&new_dir, description)?;
            println!("initialized {}", session.id);
            return Ok(());
        }
    };

    if let Some(current) = read_session(&spenser_dir) {
        if current.is_open() {
            bail!("current session is open, run `spenser close` first");
        }
        archive_session(&spenser_dir)?;
    }

    let session = create_session(&spenser_dir, description)?;
    println!("archived previous session, initialized {}", session.id);
    Ok(())
}
