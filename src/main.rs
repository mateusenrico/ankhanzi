pub mod lib;

use lib::{AnkiConnect, Dict, DECKS};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let anki = AnkiConnect::new();

    let dict = Dict::init();

    anki.create_decks(&DECKS.values().collect::<Vec<_>>())
        .await?
        .add_notes(&dict.list)
        .await?;

    Ok(())
}
