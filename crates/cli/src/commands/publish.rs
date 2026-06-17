use anyhow::{bail,Result};
use std::env;

use core::dir::{find_spenser_dir, read_config, read_session, write_session, read_anchor};
use registry::vcs::open_vcs;
use registry::producer::open_producer;
use registry::slicer::open_slicer;
use types::{Card, Round};

pub fn execute(commit: String) -> Result<()> {
    let current_dir = env::current_dir()?;
    let spenser_dir = find_spenser_dir(&current_dir)
        .ok_or_else(|| anyhow::anyhow!("no .spenser found, run `spenser init` first"))?;

    let session = read_session(&spenser_dir)
        .ok_or_else(|| anyhow::anyhow!("no active session, run `spenser init` first"))?;
    if !session.is_open() {
        bail!("session is closed");
    }

    let anchor = read_anchor(&spenser_dir)
        .ok_or_else(|| anyhow::anyhow!("no anchor exists, run `spenser anchor` first"))?;

    let config = read_config(&spenser_dir)?;
    let vcs = open_vcs(&config, ".")?;
    let change = vcs.resolve(&commit)?;

    let producer = open_producer(&config, ".")?;
    let schema_diff = producer.produce(&anchor, &change)?;

    let slicer = open_slicer(&config)?;
    let card_contents = slicer.slice(&schema_diff);

    for content in &card_contents {
        println!("{content:#?}");
    }

    let mut session = session;

    let cards: Vec<Card> = card_contents.into_iter()
        .map(Card::new)
        .collect();
    
    let mut round = Round::new(anchor, change, schema_diff);
    round.card_refs = cards.iter().map(|c| c.id.clone()).collect();

    let card_count = round.card_refs.len();
    for card in cards {
        session.register_card(card);
    }
    session.add_round(round);

    write_session(&spenser_dir, &session)?;

    println!("published round {} with {} cards", 
        session.rounds.len(), 
        card_count);

    Ok(())
}
