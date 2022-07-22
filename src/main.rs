pub mod lib;

use lib::Dict;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Dict::from_complete_db()
        .await?
        .anki_create_initial_decks()
        .await?
        .exec(|| println!("Decks"))
        .anki_create_all_notes()
        .await?
        .exec(|| println!("Cards"))
        .anki_suspend_notes()
        .await?
        .exec(|| println!("Suspended"))
        .anki_sync()
        .await?
        .exec(|| println!("Finalizado"));

    Ok(())
}
