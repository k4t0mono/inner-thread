use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Oracle {
    name: String,
    decks: Vec<Deck>,
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


#[derive(Debug, Deserialize, Serialize)]
pub struct Meaning {
    r#type: Option<String>,
    text: Option<String>,
    keywords: Vec<String>,
}