use std::io;
use std::io::prelude::*;
use std::fs::File;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Oracle {
    name: String,
    pub decks: Vec<Deck>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Deck {
    name: String,
    description: Option<String>,
    cards: Vec<Card>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    pub id: u32,
    pub deck: String,
    pub name: String,
    pub meanings: Vec<Meaning>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Meaning {
    r#type: Option<String>,
    text: Option<String>,
    keywords: Vec<String>,
}


pub fn load_oracle(path: String) -> io::Result<Oracle> {
    let mut fl = File::open(path)?;
    let mut buff = String::new();

    fl.read_to_string(&mut buff)?;

    let oracle: Oracle = toml::from_str(&buff)?;

    Ok(oracle)
}


pub fn prepare_decks(oracle: Oracle) -> Vec<Card> {
    let mut cards: Vec<Card> = vec!();

    for deck in oracle.decks.into_iter() {
        for card in deck.cards.into_iter() {
            cards.push(card);
        }
    }

    cards
}