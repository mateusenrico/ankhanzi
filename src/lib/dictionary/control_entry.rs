use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ControlEntry {
    pub time: String,
    pub comment: String,
    pub uuid: String,
    pub done: bool,
}
