use colored::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

pub const LOCAL_ADDRESS: &str = "localhost:8080";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    UserMessage(String, SocketAddr),
    SystemMessage(String),
}

impl Message {
    pub fn print(&self) {
        match self {
            Message::UserMessage(msg, address) => {
                println!("[{}]:{}", address.to_string().blue(), msg)
            }
            Message::SystemMessage(msg) => println!("[{}]:{}", "SYSTEM".red(), msg),
        }
    }

    pub fn serialize(&self) -> String {
        let mut str = serde_json::to_string(self).unwrap();
        str.push('\n');
        str
    }

    pub fn deserialize(data: String) -> Message {
        serde_json::from_str(data.trim()).unwrap()
    }
}
