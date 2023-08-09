use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    text: String,
    color: String,
}

impl Message {
    pub fn new(text: String, color: String) -> Message {
        Message { text, color }
    }
}
