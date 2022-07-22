use crate::lib::dictionary::{ControlEntry, Hanzi};

use futures::TryStreamExt;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use std::collections::HashMap;

static URI: &'static str = "cluster0.j3dt7gs.mongodb.net/?retryWrites=true&w=majority";

pub struct Mongo {
    pub db: Database,
}

impl Mongo {
    async fn client() -> Result<Client, Box<dyn std::error::Error>> {
        dotenv::from_filename("vars.env").ok();
        let db_vars: HashMap<String, String> = std::env::vars()
            .filter(|f| f.0 == "DB_USER" || f.0 == "DB_PASSWORD")
            .collect();

        let client_options = ClientOptions::parse(format!(
            "mongodb+srv://{}:{}@{}",
            db_vars["DB_USER"], db_vars["DB_PASSWORD"], URI
        ))
        .await?;

        Ok(Client::with_options(client_options)?)
    }

    pub async fn connect() -> Result<Mongo, Box<dyn std::error::Error>> {
        let client = Mongo::client().await?;
        let db = client.database("Hanzi");

        Ok(Mongo { db })
    }

    pub async fn fetch_control(self) -> Result<Vec<ControlEntry>, Box<dyn std::error::Error>> {
        Ok(self
            .db
            .collection::<ControlEntry>("Admin-dev")
            .find(None, None)
            .await?
            .try_collect::<Vec<ControlEntry>>()
            .await?)
    }

    pub async fn insert_notes(
        self,
        notes: &Vec<Hanzi>,
    ) -> Result<Mongo, Box<dyn std::error::Error>> {
        self.db
            .collection::<Hanzi>("Dev")
            .insert_many(notes, None)
            .await?;
        Ok(self)
    }

    pub async fn get_all(self) -> Result<Vec<Hanzi>, Box<dyn std::error::Error>> {
        Ok(self
            .db
            .collection::<Hanzi>("Dev")
            .find(None, None)
            .await?
            .try_collect::<Vec<Hanzi>>()
            .await?)
    }

    pub async fn get_by_uuid(self, uuid: &String) -> Result<Hanzi, Box<dyn std::error::Error>> {
        Ok(self
            .db
            .collection::<Hanzi>("Dev")
            .find_one(Some(doc! {"uuid": uuid}), None)
            .await?
            .expect("Unable to unwrap"))
    }
}
