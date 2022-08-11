use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Event {
    Join { room: String },
    Message { to: String, msg: String },
}
