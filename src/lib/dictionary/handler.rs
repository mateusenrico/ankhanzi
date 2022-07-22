use crate::lib::{
    addons::RdFile,
    dictionary::{ankiconnect::AnkiConnect, hanzi::*, ControlEntry, Mongo},
};

use phf::phf_map;
use std::{
    fs::File,
    io::Read,
    path::Path,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

static PATH_DICT: &'static str = "./DICTIONARY.json";

pub struct Dict {
    pub list: Vec<Hanzi>,
    pub exportable: bool,
}

impl Dict {
    pub async fn loop_search() -> Result<(), Box<dyn std::error::Error>> {
        let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();

        thread::spawn(move || loop {
            tx.send(()).unwrap();
            thread::sleep(Duration::new(5, 0));
        });

        loop {
            rx.recv().unwrap();
            Dict::from_control().await?.anki_unsuspend_notes().await?;
        }
    }

    pub fn from_static() -> Dict {
        let mut data = String::new();
        File::rdfile(PATH_DICT).read_to_string(&mut data).unwrap();

        let list: Vec<Hanzi> = serde_json::from_str(&data).expect("Unable to read");
        Dict {
            list,
            exportable: true,
        }
    }

    pub async fn from_complete_db() -> Result<Dict, Box<dyn std::error::Error>> {
        Ok(Dict {
            list: Dict::create_mongo().await?.get_all().await?,
            exportable: false,
        })
    }

    pub async fn from_db_by_ids(ids: Vec<String>) -> Result<Dict, Box<dyn std::error::Error>> {
        let mut list: Vec<Hanzi> = Vec::new();

        for id in ids {
            list.push(Mongo::connect().await?.get_by_uuid(&id).await?);
        }

        Ok(Dict {
            list,
            exportable: false,
        })
    }

    pub async fn from_control() -> Result<Dict, Box<dyn std::error::Error>> {
        let changed = Dict::get_mongo_control()
            .await?
            .into_iter()
            .filter(|e| !e.done)
            .map(|e| e.uuid)
            .collect::<Vec<String>>();

        Dict::from_db_by_ids(changed).await
    }

    pub const DECKS: phf::Map<&'static str, &'static str> = phf_map! {
        "ROOT" => "Ankhanzi",
        "HSK-v2002" => "Ankhanzi::HSK-v2002",
        "HSK.1" => "Ankhanzi::HSK-v2002::HSK.1",
        "HSK.2" => "Ankhanzi::HSK-v2002::HSK.2",
        "HSK.3" => "Ankhanzi::HSK-v2002::HSK.3",
        "HSK.4" => "Ankhanzi::HSK-v2002::HSK.4",
        "HSK.5" => "Ankhanzi::HSK-v2002::HSK.5",
        "HSK.6" => "Ankhanzi::HSK-v2002::HSK.6",
        "NPCR" => "Ankhanzi::NPCR",
        "NPCR.1" => "Ankhanzi::NPCR::NPCR.1",
        "NPCR.2" => "Ankhanzi::NPCR::NPCR.2",
        "NPCR.3" => "Ankhanzi::NPCR::NPCR.3",
        "NPCR.4" => "Ankhanzi::NPCR::NPCR.4",
        "NPCR.5" => "Ankhanzi::NPCR::NPCR.5",
        "NPCR.6" => "Ankhanzi::NPCR::NPCR.6",
        "HSK-v2021" => "Ankhanzi::HSK-v2021",
        "HSK3.1" => "Ankhanzi::HSK-v2021::HSK3.1",
        "HSK3.2" => "Ankhanzi::HSK-v2021::HSK3.2",
        "HSK3.3" => "Ankhanzi::HSK-v2021::HSK3.3",
        "HSK3.4" => "Ankhanzi::HSK-v2021::HSK3.4",
        "HSK3.5" => "Ankhanzi::HSK-v2021::HSK3.5",
        "HSK3.6" => "Ankhanzi::HSK-v2021::HSK3.6",
        "HSK3.7-9" => "Ankhanzi::HSK-v2021::HSK3.7-9",
        "OWN" => "Ankhanzi::OWN",
        "UNKOWN" => "Ankhanzi::UNKNOWN"
    };

    pub fn get_deck_names() -> Vec<String> {
        Dict::DECKS
            .values()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
    }

    pub async fn create_mongo() -> Result<Mongo, Box<dyn std::error::Error>> {
        Mongo::connect().await
    }

    pub async fn get_mongo_control() -> Result<Vec<ControlEntry>, Box<dyn std::error::Error>> {
        Dict::create_mongo().await?.fetch_control().await
    }

    pub async fn mongo_insert_all_notes(self) -> Result<Dict, Box<dyn std::error::Error>> {
        Dict::create_mongo().await?.insert_notes(&self.list).await?;

        Ok(self)
    }

    pub fn exec(self, f: impl Fn() -> ()) -> Dict {
        f();
        self
    }

    pub fn create_anki() -> AnkiConnect {
        AnkiConnect::new()
    }

    pub async fn anki_create_initial_decks(self) -> Result<Dict, Box<dyn std::error::Error>> {
        Dict::create_anki()
            .create_decks(Dict::get_deck_names())
            .await?;

        Ok(self)
    }

    pub async fn anki_create_all_notes(self) -> Result<Dict, Box<dyn std::error::Error>> {
        Dict::create_anki().add_notes(&self.list).await?;

        Ok(self)
    }

    pub async fn anki_suspend_notes(self) -> Result<Dict, Box<dyn std::error::Error>> {
        let mut anki = Dict::create_anki();
        let notes = anki
            .find_notes(
                self.list
                    .to_owned()
                    .into_iter()
                    .map(|e| e.uuid)
                    .collect::<Vec<String>>(),
            )
            .await?;

        anki.suspend_notes(notes).await?;
        Ok(self)
    }

    pub async fn anki_unsuspend_notes(self) -> Result<Dict, Box<dyn std::error::Error>> {
        let mut anki = Dict::create_anki();
        let notes = anki
            .find_notes(
                self.list
                    .to_owned()
                    .into_iter()
                    .map(|e| e.uuid)
                    .collect::<Vec<String>>(),
            )
            .await?;

        anki.unsuspend_notes(notes).await?;
        Ok(self)
    }

    pub async fn anki_sync(self) -> Result<Dict, Box<dyn std::error::Error>> {
        Dict::create_anki().sync().await?;
        Ok(self)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self.list).expect("Unable to create list")
    }

    pub fn export(&self) {
        std::fs::write(Path::new(&PATH_DICT), self.to_json()).expect("Unable to write");
    }

    pub fn set_export(&mut self, flag: bool) {
        self.exportable = flag;
    }
}

impl Drop for Dict {
    fn drop(&mut self) {
        if self.exportable {
            self.export();
            println!("Re-exportado!");
        } else {
            println!("Droppado sem salvar");
        }
    }
}
