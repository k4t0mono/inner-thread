extern crate serde;

mod oracle;

use std::io;
use std::io::prelude::*;
use std::fs::File;

const FILE_PATH: &str = "test/major.toml";


fn main() {
    println!("Yo");

    let deck = load_oracle()
        .expect("Error on 'load_oracle'");
    
    println!("{:?}", deck);
}


fn load_oracle() -> io::Result<oracle::Oracle> {
    let mut fl = File::open(FILE_PATH)?;
    let mut buff = String::new();

    fl.read_to_string(&mut buff)?;

    let oracle: oracle::Oracle = toml::from_str(&buff)?;

    Ok(oracle)
}