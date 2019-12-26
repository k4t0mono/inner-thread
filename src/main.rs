extern crate serde;

mod oracle;

const FILE_PATH: &str = "test/major.toml";


fn main() {
    println!("Yo");

    let oracle_ = oracle::load_oracle(FILE_PATH.to_string())
        .expect("Error on 'load_oracle'");
    
    let mut cards = oracle::prepare_decks(oracle_);
    draw_cards(&mut cards, 3);
    // for i in 0..5 {
    //     println!("{}", i);
    // }
}


fn draw_cards(cards: &mut Vec<oracle::Card>, qnt: usize) {
    use rand::seq::SliceRandom;

    let sample: Vec<_> = cards
        .choose_multiple(&mut rand::thread_rng(), qnt)
        .collect();
    
    inspect_cards(sample);
}


fn inspect_cards(cards: Vec<&oracle::Card>) {
    for card in cards.iter() {
        print!("{}, ", card.name);
    }
    println!("");
}