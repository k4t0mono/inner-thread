extern crate serde;

mod oracle;

const FILE_PATH: &str = "test/major.toml";


fn main() {
    let oracle_ = oracle::load_oracle(FILE_PATH.to_string())
        .expect("Error on 'load_oracle'");
    
    let cards = oracle::prepare_decks(oracle_);
    let draw = draw_cards(&cards, 3);

    inspect_cards(draw);
}


fn draw_cards(cards: &Vec<oracle::Card>, qnt: usize) -> Vec<&oracle::Card> {
    use rand::seq::SliceRandom;

    cards
        .choose_multiple(&mut rand::thread_rng(), qnt)
        .collect()
}


fn inspect_cards(cards: Vec<&oracle::Card>) {
    for card in cards.iter() {
        print!("{}: ", card.get_name());

        for m in card.get_meaning(0).unwrap().get_keywords() {
            print!("{}, ", m);
        }

        println!("");
    }
}