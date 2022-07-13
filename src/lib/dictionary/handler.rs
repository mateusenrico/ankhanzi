use crate::lib::{addons::RdFile, dictionary::hanzi::*};

use std::{fs::File, io::Read, path::Path};
// use uuid::{uuid, Uuid};

static PATH_DICT: &'static str = "./static/DICT.json";
static WRITE_NAME: &'static str = "DICT";

pub struct Dict {
    pub list: Vec<Hanzi>,
}

impl Dict {
    pub fn init() -> Dict {
        let mut data = String::new();
        File::rdfile(PATH_DICT).read_to_string(&mut data).unwrap();

        let list: Vec<Hanzi> = serde_json::from_str(&data).expect("Unable to read");
        let dict = Dict { list };
        dict
    }

    pub fn json(&self) -> String {
        serde_json::to_string_pretty(&self.list).expect("Unable to parse")
    }

    pub fn export(self) {
        std::fs::write(
            Path::new(&format!("./static/OUT_{}.json", WRITE_NAME)),
            self.json(),
        )
        .expect("Unable to write");
    }

    pub fn count_discovered(&self) -> usize {
        self.list
            .iter()
            .filter(|x| x.was_discovered)
            .collect::<Vec<_>>()
            .len()
    }
}
