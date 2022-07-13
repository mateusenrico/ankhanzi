use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub pinyin: String,
    pub definition: Vec<String>,
    pub origin: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Internal {
    pub traditional: String,
    pub specifications: Vec<Entry>,
    pub number_specifications: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hanzi {
    pub uuid: String,
    pub simplified: String,
    pub entries: Vec<Internal>,
    pub number_entries: u8,
    pub tags: Vec<String>,
    pub is_known: bool,
    pub on_anki: bool,
    pub single: bool,
    pub composed_by: Vec<String>,
    pub composes: Vec<String>,
    pub is_measure: bool,
    pub measured_by: Vec<String>,
    pub measures: Vec<String>,
    pub erhua_version: bool,
    pub was_discovered: bool,
}
