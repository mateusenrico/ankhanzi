use crate::lib::Dict;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

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

impl Hanzi {
    fn anki_note(&self) -> HashMap<String, String> {
        let mut output = HashMap::new();

        output.insert("uuid".to_string(), self.uuid.clone());
        output.insert("simplified".to_string(), self.simplified.clone());
        output.insert(
            "traditionals".to_string(),
            self.entries
                .iter()
                .map(|x| x.traditional.clone())
                .collect::<Vec<String>>()
                .join("|"),
        );

        output.insert(
            "pinyins".to_string(),
            self.entries
                .iter()
                .map(|f| {
                    f.specifications
                        .iter()
                        .map(|g| g.pinyin.clone())
                        .collect::<Vec<String>>()
                        .join("|")
                })
                .collect::<Vec<String>>()
                .join("@"),
        );
        output.insert(
            "definitions".to_string(),
            self.entries
                .iter()
                .map(|f| {
                    f.specifications
                        .iter()
                        .map(|g| {
                            g.definition
                                .iter()
                                .map(|h| h.clone())
                                .collect::<Vec<String>>()
                                .join("|")
                        })
                        .collect::<Vec<String>>()
                        .join("@")
                })
                .collect::<Vec<String>>()
                .join("#"),
        );

        output
    }

    fn find_deck(&self) -> String {
        if self.tags.contains(&"HSK.1".to_string()) {
            Dict::DECKS.get("HSK.1").unwrap().to_string()
        } else if self.tags.contains(&"HSK.2".to_string()) {
            Dict::DECKS.get("HSK.2").unwrap().to_string()
        } else if self.tags.contains(&"HSK.3".to_string()) {
            Dict::DECKS.get("HSK.3").unwrap().to_string()
        } else if self.tags.contains(&"HSK.4".to_string()) {
            Dict::DECKS.get("HSK.4").unwrap().to_string()
        } else if self.tags.contains(&"HSK.5".to_string()) {
            Dict::DECKS.get("HSK.5").unwrap().to_string()
        } else if self.tags.contains(&"HSK.6".to_string()) {
            Dict::DECKS.get("HSK.6").unwrap().to_string()
        } else if self.tags.contains(&"NPCR.1".to_string()) {
            Dict::DECKS.get("NPCR.1").unwrap().to_string()
        } else if self.tags.contains(&"NPCR.2".to_string()) {
            Dict::DECKS.get("NPCR.2").unwrap().to_string()
        } else if self.tags.contains(&"NPCR.3".to_string()) {
            Dict::DECKS.get("NPCR.3").unwrap().to_string()
        } else if self.tags.contains(&"NPCR.4".to_string()) {
            Dict::DECKS.get("NPCR.4").unwrap().to_string()
        } else if self.tags.contains(&"NPCR.5".to_string()) {
            Dict::DECKS.get("NPCR.5").unwrap().to_string()
        } else if self.tags.contains(&"NPCR.6".to_string()) {
            Dict::DECKS.get("NPCR.6").unwrap().to_string()
        } else if self.tags.contains(&"HSK3.1".to_string()) {
            Dict::DECKS.get("HSK3.1").unwrap().to_string()
        } else if self.tags.contains(&"HSK3.2".to_string()) {
            Dict::DECKS.get("HSK3.2").unwrap().to_string()
        } else if self.tags.contains(&"HSK3.3".to_string()) {
            Dict::DECKS.get("HSK3.3").unwrap().to_string()
        } else if self.tags.contains(&"HSK3.4".to_string()) {
            Dict::DECKS.get("HSK3.4").unwrap().to_string()
        } else if self.tags.contains(&"HSK3.5".to_string()) {
            Dict::DECKS.get("HSK3.5").unwrap().to_string()
        } else if self.tags.contains(&"HSK3.6".to_string()) {
            Dict::DECKS.get("HSK3.6").unwrap().to_string()
        } else if self.tags.contains(&"HSK3.7-9".to_string()) {
            Dict::DECKS.get("HSK3.7-9").unwrap().to_string()
        } else if self.tags.len() == 0 {
            Dict::DECKS.get("UNKOWN").unwrap().to_string()
        } else {
            Dict::DECKS.get("OWN").unwrap().to_string()
        }
    }

    pub fn to_anki(&self) -> serde_json::Value {
        let deck = self.find_deck();
        let tags = if self.tags.len() != 0 {
            self.tags.clone()
        } else {
            vec!["OWN.Unknown".to_string()]
        };

        json!({
                "deckName": deck,
                "modelName": "Ankhanzi",
                "fields": self.anki_note(),
                "tags": tags,
                "options": {
                    "allowDuplicate": false,
                    "duplicateScope": "deck",
                    "duplicateScopeOptions": {
                        "deckName": deck,
                        "checkChildren": false,
                        "checkAllModels": false
                    }
                }
            }
        )
    }

    pub fn bson(&self) -> bson::Bson {
        bson::to_bson(&self).expect("Unable to parse")
    }

    pub fn json(&self) -> String {
        serde_json::to_string(&self).expect("Unable to parse Hanzi")
    }
}
