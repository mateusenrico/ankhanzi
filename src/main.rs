pub mod lib;

use indicatif::ProgressBar;
use lib::BarPreCreate;
use serde_json::json;

use crate::lib::{Dict, DECKS};

static URL: &'static str = "http://localhost:8765/";

async fn create_decks(client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
    for v in DECKS.values() {
        let args = json!({
            "action": "createDeck",
            "version": 6,
            "params": {
                "deck": v
            }
        });

        client
            .post(URL)
            .json(&args)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // create_decks(&client).await?;

    let dict = Dict::init();

    let bar = ProgressBar::create(
        dict.list
            .iter()
            .filter(|x| x.was_discovered)
            .collect::<Vec<_>>()
            .len() as u64,
    );
    for word in dict.list.iter().filter(|x| x.was_discovered) {
        let args = json!({
            "action": "addNote",
            "version": 6,
            "params": {
                "note": word.to_anki()
            }
        });

        client
            .post(URL)
            .json(&args)
            .send()
            .await
            .expect(&format!("ERRO NO CARACTER: {:#?}", word));

        bar.inc(1);
    }

    let args = json!({
        "action": "sync",
        "version": 6,
    });

    client.post(URL).json(&args).send().await?;

    Ok(())
}
