use crate::lib::{filepath::RdFile, hanzi::*, pbar::BarPreCreate};

use indicatif::ProgressBar;
use std::{collections::HashMap, fs::File, path::Path};
use uuid::Uuid;
// use uuid::{uuid, Uuid};

static PATH_CEDICT: &'static str = "./static/cedict.txt";
static WRITE_NAME: &'static str = "DICT-2";

pub struct Dict {
    pub list: HashMap<String, Hanzi>,
    bar: ProgressBar,
}

impl Dict {
    pub fn new() -> Dict {
        let list =
            cedict::parse_reader(File::rdfile(PATH_CEDICT)).collect::<Vec<cedict::DictEntry>>();
        let pb = ProgressBar::create(list.len() as u64);

        let mut dict = Dict {
            list: HashMap::new(),
            bar: pb,
        };

        list.into_iter().for_each(|e| {
            dict.populate(e);
            dict.bar.inc(1);
        });

        dict
    }

    fn populate(&mut self, e: cedict::DictEntry) {
        let key = format!("{}", e.simplified());

        let entry = Entry {
            pinyin: e.pinyin().to_string(),
            definition: e
                .definitions()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            origin: "CEDICT-202207".to_string(),
        };

        match self.list.get_mut(&key) {
            Some(x) => match x
                .to_owned()
                .entries
                .into_iter()
                .position(|r| r.traditional == e.traditional())
            {
                Some(y) => {
                    x.entries[y].specifications.push(entry);
                    x.entries[y].number_specifications += 1;
                }
                None => {
                    let inter = Internal {
                        traditional: e.traditional().to_string(),
                        specifications: vec![entry],
                        number_specifications: 1,
                    };

                    x.entries.push(inter);
                    x.number_entries += 1;
                }
            },
            None => {
                let inter = Internal {
                    traditional: e.traditional().to_string(),
                    specifications: vec![entry],
                    number_specifications: 1,
                };

                let hanzi = Hanzi {
                    uuid: Uuid::new_v4().to_string(),
                    simplified: e.simplified().to_string(),
                    entries: vec![inter],
                    tags: vec![],
                    number_entries: 1,
                    is_known: false,
                    on_anki: false,
                    single: e
                        .simplified()
                        .to_string()
                        .chars()
                        .collect::<Vec<char>>()
                        .len()
                        == 1,
                    composed_by: vec![],
                    composes: vec![],
                    is_measure: false,
                    measured_by: vec![],
                    measures: vec![],
                    erhua_version: e.simplified().to_string().chars().last().unwrap() == '儿'
                        && e.pinyin().to_string().ends_with(" r5"),
                    was_discovered: false,
                };

                self.list.insert(key, hanzi);
            }
        }
    }

    pub fn json(&self) -> String {
        serde_json::to_string_pretty(&Vec::from_iter(self.list.values())).expect("Unable to parse")
    }

    pub fn export(self) {
        std::fs::write(
            Path::new(&format!("./static/out/{}.json", WRITE_NAME)),
            self.json(),
        )
        .expect("Unable to write");
    }
}
