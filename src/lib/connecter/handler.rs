use indicatif::ProgressBar;
use serde_json::{json, Value};

use crate::lib::{addons::BarPreCreate, dictionary::Hanzi};

static URL: &'static str = "http://localhost:8765/";

pub struct AnkiConnect {
    pub handler: reqwest::Client,
    pub bar: Option<indicatif::ProgressBar>,
    pub action: Option<String>,
    pub params: Option<Value>,
    pub args: Option<Value>,
}

impl AnkiConnect {
    pub fn new() -> AnkiConnect {
        AnkiConnect {
            handler: reqwest::Client::new(),
            bar: None,
            action: None,
            params: None,
            args: None,
        }
    }

    fn create_bar(&mut self, len: &usize) {
        self.bar = Some(ProgressBar::create(len.to_owned() as u64));
    }

    fn reset_bar(&mut self) {
        self.bar = None;
    }

    fn set_args(&mut self) {
        if let Some(act) = &self.action {
            if let Some(params) = &self.params {
                self.args = Some(json!({
                    "version": 6,
                    "action": act,
                    "params": params,
                }));
            } else {
                self.args = Some(json!({
                    "version": 6,
                    "action": act,
                }));
            }
        }
    }

    pub async fn sync(mut self) -> Result<AnkiConnect, Box<dyn std::error::Error>> {
        self.action = Some("sync".to_string());
        self.params = None;
        self.req().await?;

        Ok(self)
    }

    pub async fn add_notes(
        mut self,
        list: &Vec<Hanzi>,
    ) -> Result<AnkiConnect, Box<dyn std::error::Error>> {
        self.create_bar(&list.len());
        for word in list.iter() {
            self.action = Some("addNote".to_string());
            self.params = Some(json!({
                "note": word.to_anki()
            }));
            self.req().await?;
        }
        self.reset_bar();
        Ok(self)
    }

    pub async fn create_decks(
        mut self,
        list: &Vec<&&str>,
    ) -> Result<AnkiConnect, Box<dyn std::error::Error>> {
        self.create_bar(&list.len());
        for deck in list.iter() {
            self.action = Some("createDeck".to_string());
            self.params = Some(json!({ "deck": deck }));
            self.req().await?;
        }

        self.reset_bar();
        Ok(self)
    }

    pub async fn req(&mut self) -> Result<Value, reqwest::Error> {
        self.set_args();
        let res = self
            .handler
            .post(URL)
            .json(&self.args)
            .send()
            .await?
            .json::<Value>()
            .await;

        if let Some(bar) = &mut self.bar {
            bar.inc(1);
        }

        res
    }
}
