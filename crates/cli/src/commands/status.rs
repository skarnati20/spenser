use anyhow::Result;
use std::env;

use core::dir::{find_spenser_dir, read_session, read_anchor};

pub fn execute() -> Result<()> {
    let cwd = env::current_dir()?;
    let dir = find_spenser_dir(&cwd)
        .ok_or_else(|| anyhow::anyhow!("no .spenser found"))?;

    let session = read_session(&dir)
        .ok_or_else(|| anyhow::anyhow!("no active session, run `spenser init` first"))?;
    let anchor = read_anchor(&dir);

    println!("Session: {}  ({})", session.id, 
        if session.is_open() { "Open" } else { "Closed" });
    println!("  {}", session.description);

    if let Some(hash) = anchor {
        println!("  anchor: {}", hash.short());
    } else {
        println!("  anchor: not set");
    }

    println!("  {} round(s), {} card(s)", 
        session.rounds.len(), 
        session.cards.len());

    if session.rounds.is_empty() {
        println!("\n  no rounds yet — run `spenser publish`");
        return Ok(());
    }

    for (i, round) in session.rounds.iter().enumerate() {
        println!("\n  Round {} ({})", i + 1, round.id);
        println!("    {} → {}", 
            round.base_hash.short(), 
            round.head_hash.short());
        println!("    {} card(s)", round.card_refs.len());
    }

    if !session.cards.is_empty() {
        println!("\n  Cards:");
        for (id, card) in &session.cards {
            let path = card.content.region_ref.path.display();
            let range = &card.content.region_ref.range;
            let responses = card.thread.len();
            println!("    [{}] {}:{}-{}  ({} response(s))",
                &id.as_str()[..8],
                path,
                range.new_start,
                range.new_start + range.new_lines,
                responses);
        }
    }

    Ok(())
}