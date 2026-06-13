use std::collections::HashMap;

use crate::card::Card;
use crate::ids::{CardId, ChangeId};
use crate::round::Round;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SessionStatus {
    Open,
    Closed,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Session {
    pub id: ChangeId,
    pub description: String,
    pub status: SessionStatus,
    pub rounds: Vec<Round>,
    pub cards: HashMap<CardId, Card>,
}

impl Session {
    pub fn new(description: String) -> Self {
        Self {
            id: ChangeId::new(),
            description,
            status: SessionStatus::Open,
            rounds: Vec::new(),
            cards: HashMap::new(),
        }
    }

    pub fn is_open(&self) -> bool {
        matches!(self.status, SessionStatus::Open)
    }

    pub fn close(&mut self) {
        self.status = SessionStatus::Closed;
    }

    pub fn latest_round(&self) -> Option<&Round> {
        self.rounds.last()
    }
 
    pub fn add_round(&mut self, round: Round) {
        self.rounds.push(round);
    }
 
    pub fn register_card(&mut self, card: Card) {
        self.cards.insert(card.id.clone(), card);
    }
 
    pub fn get_card(&self, id: &CardId) -> Option<&Card> {
        self.cards.get(id)
    }
 
    pub fn get_card_mut(&mut self, id: &CardId) -> Option<&mut Card> {
        self.cards.get_mut(id)
    }
}