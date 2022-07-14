use crate::lib::{addons::RdFile, dictionary::hanzi::*};

use std::{fs::File, io::Read, path::Path};
// use uuid::{uuid, Uuid};

static PATH_DICT: &'static str = "./static/DICTIONARY.json";

pub struct Dict {
    pub count: u64,
    pub list: Vec<Hanzi>,
    pub exportable: bool,
}

impl Dict {
    pub fn init() -> Dict {
        let mut data = String::new();
        File::rdfile(PATH_DICT).read_to_string(&mut data).unwrap();

        let list: Vec<Hanzi> = serde_json::from_str(&data).expect("Unable to read");
        let dict = Dict {
            count: list.len() as u64,
            list,
            exportable: true,
        };

        dict
    }

    pub fn json(&self) -> String {
        serde_json::to_string_pretty(&self.list).expect("Unable to parse")
    }

    pub fn export(&self) {
        std::fs::write(Path::new(&PATH_DICT), self.json()).expect("Unable to write");
    }

    pub fn set_export(&mut self, flag: bool) {
        self.exportable = flag;
    }

    pub fn count_discovered(&self) -> usize {
        self.list
            .iter()
            .filter(|x| x.was_discovered)
            .collect::<Vec<_>>()
            .len()
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
