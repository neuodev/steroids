use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    Join { room: String },
    Message { to: String, msg: String },
    Register {
        username: String,
        password: String,
    },
    Login {
        username: String,
        password: String,
    },
    AddFriend {
        friend_id: u32,
    }
}
