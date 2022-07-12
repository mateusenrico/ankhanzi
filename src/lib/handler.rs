use crate::lib::{filepath::RdFile, hanzi::*, pbar::BarPreCreate};

use indicatif::ProgressBar;
use std::{collections::HashMap, fs::File, io::Read, path::Path};
use uuid::Uuid;
// use uuid::{uuid, Uuid};

static PATH_DICT: &'static str = "./static/saida.json";
static WRITE_NAME: &'static str = "DICT-3";

pub struct Dict {
    pub list: Vec<Hanzi>,
    bar: Option<ProgressBar>,
}

impl Dict {
    pub fn init() -> Dict {
        let mut data = String::new();
        File::rdfile(PATH_DICT).read_to_string(&mut data).unwrap();

        let list: Vec<Hanzi> = serde_json::from_str(&data).expect("Unable to read");
        let dict = Dict {
            list: list,
            bar: None,
        };
        dict
    }

    pub fn json(&self) -> String {
        serde_json::to_string_pretty(&self.list).expect("Unable to parse")
    }

    pub fn export(self) {
        std::fs::write(
            Path::new(&format!("./static/out/{}.json", WRITE_NAME)),
            self.json(),
        )
        .expect("Unable to write");
    }
}
