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
    id: u32,
    deck: String,
    name: String,
    meanings: Vec<Meaning>,
}

impl Card {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_deck(&self) -> &String {
        &self.deck
    }

    pub fn get_meaning(&self, index: usize) -> Option<&Meaning> {
        self.meanings.get(index)
    }

    
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Meaning {
    r#type: Option<String>,
    text: Option<String>,
    keywords: Vec<String>,
}

impl Meaning {
    pub fn get_type(&self) -> &Option<String> {
        &self.r#type
    }

    pub fn get_text(&self) -> &Option<String> {
        &self.text
    }

    pub fn get_keywords(&self) -> &Vec<String> {
        &self.keywords
    }
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