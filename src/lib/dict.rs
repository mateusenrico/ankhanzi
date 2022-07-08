use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};
use indicatif::ProgressBar;

use crate::lib::pbar::BarPreCreate;

enum OutStyle {
    List(String),
    Map(String),
}

impl OutStyle {
    fn new(name: &str, out: &str) -> OutStyle {
        let name = name.trim().to_uppercase();

        if out.trim().to_lowercase() == "list" {
            OutStyle::List(format!("List-{}.json", name))
        } else if out == "map" {
            OutStyle::Map(format!("Map-{}.json", name))
        } else {
            panic!("Output choice error")
        }
    }

    fn unwrap(&self) -> &str {
        match &self {
            OutStyle::List(x) => x,
            OutStyle::Map(x) => x,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pinyin: String,
    definition: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hanzi {
    traditional: String,
    simplified: String,
    entries: Vec<Entry>,
}

pub struct Dict {
    pub list: HashMap<String, Hanzi>,
    bar: ProgressBar,
    out: OutStyle,
    pattern: String,
}

impl Dict {
    pub fn new(list: Vec<cedict::DictEntry>, name: &str, out: &str, pattern: &str) -> Dict {
        let pb = ProgressBar::create(list.len() as u64);

        let mut dict = Dict {
            list: HashMap::new(),
            bar: pb,
            out: OutStyle::new(name, out),
            pattern: pattern.to_owned(),
        };

        list.into_iter().for_each(|e| {
            dict.populate(e);
            dict.bar.inc(1);
        });

        dict
    }

    pub fn populate(&mut self, e: cedict::DictEntry) {
        let key = format!("{}{}{}", e.traditional(), self.pattern, e.simplified());

        let def = Entry {
            pinyin: e.pinyin().to_string(),
            definition: e
                .definitions()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
        };

        match self.list.get_mut(&key) {
            Some(x) => x.entries.push(def),
            None => {
                let entry = Hanzi {
                    traditional: e.traditional().to_string(),
                    simplified: e.simplified().to_string(),
                    entries: vec![def],
                };

                self.list.insert(key, entry);
            }
        }
    }

    pub fn json(&self) -> String {
        match self.out {
            OutStyle::List(_) => serde_json::to_string_pretty(&Vec::from_iter(self.list.values()))
                .expect("Unable to parse"),
            OutStyle::Map(_) => serde_json::to_string_pretty(&self.list).expect("Unable to parse"),
        }
    }

    pub fn export(self) {
        std::fs::write(
            Path::new(&format!("./out/{}", &self.out.unwrap())),
            self.json(),
        )
        .expect("Unable to write");
    }
}
